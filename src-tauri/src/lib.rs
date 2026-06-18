pub mod commands;
pub mod confuse;
pub mod providers;
pub mod sync;
pub mod vault;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AppStatus {
    pub app_name: &'static str,
    pub version: &'static str,
}

pub fn app_info() -> AppStatus {
    AppStatus {
        app_name: "VipaVault",
        version: env!("CARGO_PKG_VERSION"),
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
    fn app_info_exposes_package_version() {
        let status = app_info();

        assert_eq!(status.app_name, "VipaVault");
        assert_eq!(status.version, env!("CARGO_PKG_VERSION"));
    }
}
