# Notification Spec

## Overview

macOS 알림 센터에 알림을 표시. `tauri-plugin-notification` 사용.

---

## API

### `show_notification`

| 항목 | 내용 |
|------|------|
| 파일 | `commands/notification.rs` |
| 방식 | `tauri-plugin-notification` |
| 파라미터 | `title: String`, `body: String` |
| 반환 | `()` |
| 에러 | `String` (알림 발송 실패 시) |

```ts
await invoke("show_notification", { title: "제목", body: "내용" });
```

---

## Permissions

`capabilities/default.json`:

```json
"notification:default"
```

> macOS 알림 권한은 시스템 설정 → 알림에서 사용자가 허용해야 함.
