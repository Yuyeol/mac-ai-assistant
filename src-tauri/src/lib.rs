// ─── 앱 조립 및 실행 ──────────────────────────────────────────
// 플러그인 등록, 독 아이콘 숨기기, 트레이 설정,
// 커맨드들을 React에서 호출 가능하도록 등록.

mod commands;
mod models;
mod tray;

use commands::{actions::notification, data::notes, data::reminders};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            // macOS: 독 아이콘 숨기기 (메뉴바 전용 앱)
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            tray::setup_tray(app)?;

            // 포커스를 잃으면 팝업 숨김 (화면 밖 클릭)
            if let Some(window) = app.get_webview_window("main") {
                let win = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::Focused(false) = event {
                        let _ = win.hide();
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            notes::get_notes,
            reminders::get_reminders,
            notification::show_notification,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
