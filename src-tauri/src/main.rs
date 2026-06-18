// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    vipavault_lib::build_app()
        .run(tauri::generate_context!())
        .expect("failed to run VipaVault application");
}