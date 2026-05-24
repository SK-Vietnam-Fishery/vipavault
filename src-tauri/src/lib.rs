pub mod commands;
pub mod confuse;
pub mod providers;
pub mod sync;
pub mod vault;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AppStatus {
    pub app_name: &'static str,
    pub milestone: &'static str,
}

pub fn foundation_status() -> AppStatus {
    AppStatus {
        app_name: "VipaVault",
        milestone: "M0 Project Foundation",
    }
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::app_status])
        .run(tauri::generate_context!())
        .expect("failed to run VipaVault application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foundation_status_identifies_m0() {
        let status = foundation_status();

        assert_eq!(status.app_name, "VipaVault");
        assert_eq!(status.milestone, "M0 Project Foundation");
    }
}
