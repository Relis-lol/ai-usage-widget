use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProviderId {
    Codex,
    Claude,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProviderStatus {
    Live,
    Stale,
    Unavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    LocalAppServer,
    OfficialLocalCli,
    Unavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsageWindow {
    pub provider: ProviderId,
    pub window_id: String,
    pub display_name: String,
    pub duration_minutes: Option<i64>,
    pub used_percent: Option<f64>,
    pub remaining_percent: Option<f64>,
    pub resets_at: Option<DateTime<Utc>>,
    pub captured_at: DateTime<Utc>,
    pub freshness_seconds: i64,
    pub source_type: SourceType,
    pub state: ProviderStatus,
    pub unavailable_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProviderSnapshot {
    pub provider: ProviderId,
    pub status: ProviderStatus,
    pub captured_at: DateTime<Utc>,
    pub source_type: SourceType,
    pub windows: Vec<UsageWindow>,
    pub message: Option<String>,
}

impl UsageWindow {
    #[allow(clippy::too_many_arguments)]
    pub fn from_used(
        provider: ProviderId,
        id: &str,
        name: &str,
        duration: Option<i64>,
        used: f64,
        resets_at: Option<DateTime<Utc>>,
        captured_at: DateTime<Utc>,
        source_type: SourceType,
    ) -> Option<Self> {
        if !(0.0..=100.0).contains(&used) || duration.is_some_and(|d| d <= 0 || d > 525_600) {
            return None;
        }
        // Official local interfaces are sampled live, so freshness starts at zero.
        // A passed reset is stale instead of being shown indefinitely.
        let freshness_seconds = 0;
        let stale = resets_at.is_some_and(|r| r <= captured_at);
        Some(Self {
            provider,
            window_id: id.into(),
            display_name: name.into(),
            duration_minutes: duration,
            used_percent: Some(used),
            remaining_percent: Some((100.0 - used).clamp(0.0, 100.0)),
            resets_at,
            captured_at,
            freshness_seconds,
            source_type,
            state: if stale {
                ProviderStatus::Stale
            } else {
                ProviderStatus::Live
            },
            unavailable_reason: None,
        })
    }
}

pub fn display_name(duration: Option<i64>, fallback: &str) -> String {
    match duration {
        Some(d) if (240..=360).contains(&d) => "5 hours".into(),
        Some(d) if (9_000..=11_000).contains(&d) => "Weekly".into(),
        _ => fallback.into(),
    }
}
