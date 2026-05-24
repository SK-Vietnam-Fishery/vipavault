use crate::{foundation_status, AppStatus};

#[tauri::command]
pub fn app_status() -> AppStatus {
    foundation_status()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_status_uses_foundation_status() {
        assert_eq!(app_status(), foundation_status());
    }
}
