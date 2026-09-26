use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct AppConfig {
    pub theme: String,
    pub always_on_top: bool,
    pub close_to_tray: bool,
    pub start_with_windows: bool,
    pub refresh_minutes: u32,
    pub codex_enabled: bool,
    pub claude_enabled: bool,
    pub window_x: Option<i32>,
    pub window_y: Option<i32>,
}
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "system".into(),
            always_on_top: false,
            close_to_tray: true,
            start_with_windows: false,
            refresh_minutes: 5,
            codex_enabled: true,
            claude_enabled: true,
            window_x: None,
            window_y: None,
        }
    }
}
impl AppConfig {
    pub fn load(path: &Path) -> Self {
        fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }
    pub fn save(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|_| "Could not create app data folder")?;
        }
        let data = serde_json::to_vec_pretty(self).map_err(|_| "Could not serialize settings")?;
        fs::write(path, data).map_err(|_| "Could not save settings".into())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roundtrip() {
        let p = std::env::temp_dir().join("ai-usage-widget-config-test.json");
        let c = AppConfig {
            theme: "dark".into(),
            ..AppConfig::default()
        };
        c.save(&p).unwrap();
        assert_eq!(AppConfig::load(&p), c);
        let _ = std::fs::remove_file(p);
    }
}
