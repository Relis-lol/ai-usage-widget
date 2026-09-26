use crate::{
    model::{ProviderId, ProviderSnapshot, ProviderStatus, SourceType},
    providers::UsageProvider,
};
use chrono::Utc;
pub struct ClaudeProvider;
impl UsageProvider for ClaudeProvider {
    fn read(&self) -> Result<ProviderSnapshot, String> {
        Ok(ProviderSnapshot {
            provider: ProviderId::Claude,
            status: ProviderStatus::Unavailable,
            captured_at: Utc::now(),
            source_type: SourceType::Unavailable,
            windows: vec![],
            message: Some("No supported local usage source detected.".into()),
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn safely_reports_unavailable() {
        assert_eq!(
            ClaudeProvider.read().unwrap().status,
            ProviderStatus::Unavailable
        );
    }
}
