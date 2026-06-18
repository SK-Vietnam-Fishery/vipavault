use crate::{app_info, AppStatus};

#[tauri::command]
pub fn app_status() -> AppStatus {
    app_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_status_returns_app_info() {
        assert_eq!(app_status(), app_info());
    }
}
