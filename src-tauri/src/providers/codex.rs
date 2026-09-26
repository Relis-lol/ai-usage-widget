use crate::{
    model::{ProviderId, ProviderSnapshot, ProviderStatus, SourceType, UsageWindow, display_name},
    providers::UsageProvider,
};
use chrono::{TimeZone, Utc};
use serde::Deserialize;
use serde_json::json;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    os::windows::process::CommandExt,
    process::{Command, Stdio},
    time::Duration,
};
const CREATE_NO_WINDOW: u32 = 0x08000000;
pub struct CodexProvider;

#[derive(Debug, Deserialize)]
struct RpcResponse {
    id: Option<u64>,
    result: Option<GetRateLimitsResponse>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetRateLimitsResponse {
    rate_limits: Option<RateLimitSnapshot>,
    rate_limits_by_limit_id: Option<HashMap<String, RateLimitSnapshot>>,
}

#[derive(Debug, Deserialize)]
struct RateLimitSnapshot {
    primary: Option<RateLimitWindow>,
    secondary: Option<RateLimitWindow>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RateLimitWindow {
    used_percent: f64,
    window_duration_mins: Option<i64>,
    resets_at: Option<i64>,
}

impl UsageProvider for CodexProvider {
    fn read(&self) -> Result<ProviderSnapshot, String> {
        let mut child = Command::new("codex")
            .args(["app-server", "--stdio"])
            .creation_flags(CREATE_NO_WINDOW)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| "Codex CLI was not found".to_string())?;
        let mut input = child
            .stdin
            .take()
            .ok_or("Codex app-server input unavailable")?;
        for request in [
            json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"clientInfo":{"name":"ai-usage-widget","version":env!("CARGO_PKG_VERSION")},"capabilities":{"experimentalApi":true}}}),
            json!({"jsonrpc":"2.0","method":"initialized","params":{}}),
            json!({"jsonrpc":"2.0","id":2,"method":"account/rateLimits/read","params":{"excludeResetCreditDetails":true,"supportsLunaReserve":false}}),
        ] {
            writeln!(input, "{request}").map_err(|_| "Could not write to Codex app-server")?;
        }
        input.flush().map_err(|_| "Could not flush Codex request")?;
        let output = child
            .stdout
            .take()
            .ok_or("Codex app-server output unavailable")?;
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            for line in BufReader::new(output).lines().map_while(Result::ok) {
                if let Ok(response) = serde_json::from_str::<RpcResponse>(&line)
                    && response.id == Some(2)
                {
                    let _ = tx.send(response);
                    break;
                }
            }
        });
        let response = rx
            .recv_timeout(Duration::from_secs(12))
            .map_err(|_| "Codex usage request timed out".to_string());
        let _ = child.kill();
        parse_response(response?, Utc::now())
    }
}
fn parse_response(
    response: RpcResponse,
    captured_at: chrono::DateTime<Utc>,
) -> Result<ProviderSnapshot, String> {
    let result = response
        .result
        .ok_or("Codex app-server returned no result")?;
    let selected = result
        .rate_limits_by_limit_id
        .and_then(|mut buckets| buckets.remove("codex"))
        .or(result.rate_limits)
        .ok_or("Codex rate limits unavailable")?;
    let mut windows = Vec::new();
    for (id, fallback, raw) in [
        ("primary", "Short window", selected.primary),
        ("secondary", "Long window", selected.secondary),
    ] {
        let Some(raw) = raw else { continue };
        let used = raw.used_percent;
        let duration = raw.window_duration_mins;
        let reset = raw.resets_at.and_then(|s| Utc.timestamp_opt(s, 0).single());
        if let Some(w) = UsageWindow::from_used(
            ProviderId::Codex,
            id,
            &display_name(duration, fallback),
            duration,
            used,
            reset,
            captured_at,
            SourceType::LocalAppServer,
        ) {
            windows.push(w);
        }
    }
    if windows.is_empty() {
        return Err("Codex returned no valid usage windows".into());
    }
    let status = if windows.iter().all(|w| w.state == ProviderStatus::Stale) {
        ProviderStatus::Stale
    } else {
        ProviderStatus::Live
    };
    Ok(ProviderSnapshot {
        provider: ProviderId::Codex,
        status,
        captured_at,
        source_type: SourceType::LocalAppServer,
        windows,
        message: None,
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    fn parse(
        value: serde_json::Value,
        captured_at: chrono::DateTime<Utc>,
    ) -> Result<ProviderSnapshot, String> {
        let response = serde_json::from_value::<RpcResponse>(value)
            .expect("synthetic response must match the narrow usage schema");
        parse_response(response, captured_at)
    }

    #[test]
    fn parses_both_windows_and_remaining() {
        let now = Utc.timestamp_opt(2_000_000_000, 0).unwrap();
        let v = json!({"result":{"rateLimits":{"primary":{"usedPercent":27,"windowDurationMins":300,"resetsAt":2_000_010_000_i64},"secondary":{"usedPercent":59,"windowDurationMins":10080,"resetsAt":2_000_500_000_i64}}}});
        let s = parse(v, now).unwrap();
        assert_eq!(s.windows.len(), 2);
        assert_eq!(s.windows[0].remaining_percent, Some(73.0));
    }
    #[test]
    fn rejects_invalid_percentage() {
        let v = json!({"result":{"rateLimits":{"primary":{"usedPercent":127,"windowDurationMins":300}}}});
        assert!(parse(v, Utc::now()).is_err());
    }
    #[test]
    fn expired_is_stale() {
        let now = Utc.timestamp_opt(2_000_000_000, 0).unwrap();
        let v = json!({"result":{"rateLimits":{"primary":{"usedPercent":20,"windowDurationMins":300,"resetsAt":1_999_999_999_i64}}}});
        assert_eq!(parse(v, now).unwrap().status, ProviderStatus::Stale);
    }
    #[test]
    fn malformed_is_unavailable() {
        assert!(parse(json!({"result":{}}), Utc::now()).is_err());
    }
    #[test]
    fn prefers_named_codex_bucket() {
        let now = Utc::now();
        let reset = now.timestamp() + 3600;
        let v = json!({"result":{"rateLimits":{"primary":{"usedPercent":99}},"rateLimitsByLimitId":{"other":{"primary":{"usedPercent":88}},"codex":{"primary":{"usedPercent":11,"windowDurationMins":300,"resetsAt":reset}}}}});
        assert_eq!(parse(v, now).unwrap().windows[0].used_percent, Some(11.0));
    }

    #[test]
    fn does_not_select_an_unknown_limit_bucket() {
        let now = Utc::now();
        let v =
            json!({"result":{"rateLimitsByLimitId":{"unrelated":{"primary":{"usedPercent":5}}}}});
        assert!(parse(v, now).is_err());
    }

    #[test]
    #[ignore = "requires an installed and authenticated Codex CLI"]
    fn live_codex_probe() {
        let snapshot = CodexProvider.read().expect("live Codex usage unavailable");
        assert!(!snapshot.windows.is_empty());
        for window in snapshot.windows {
            println!(
                "{}: used={:?}% remaining={:?}% reset={:?}",
                window.display_name,
                window.used_percent,
                window.remaining_percent,
                window.resets_at
            );
        }
    }
}
