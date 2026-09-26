pub mod claude;
pub mod codex;
use crate::model::ProviderSnapshot;
pub trait UsageProvider {
    fn read(&self) -> Result<ProviderSnapshot, String>;
}
