// 메뉴바 트레이 아이콘 설정
// 클릭 시 채팅 창을 화면 하단 중앙에 표시하거나 숨김.

use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, Manager, PhysicalPosition, Position,
};

pub fn setup_tray(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .icon_as_template(true)
        .tooltip("AI 비서")
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        if let (Ok(Some(monitor)), Ok(window_size)) =
                            (window.current_monitor(), window.outer_size())
                        {
                            let scale = monitor.scale_factor();
                            let monitor_size = monitor.size();
                            let monitor_pos = monitor.position();

                            let x = monitor_pos.x
                                + ((monitor_size.width - window_size.width) / 2) as i32;
                            let y = monitor_pos.y + monitor_size.height as i32
                                - window_size.height as i32
                                - (80.0 * scale) as i32;

                            let _ = window.set_position(Position::Physical(
                                PhysicalPosition { x, y },
                            ));
                        }
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}
