# Phase 1 — 앱 뼈대 (UI & Shell)

**목표**: 메뉴바 아이콘 클릭 시 채팅 팝업 표시
**상태**: ✅ 완료

---

## Rust 구현

### 창 설정 (`tauri.conf.json`)

```json
{
  "label": "main",
  "width": 350,
  "height": 500,
  "decorations": false,
  "transparent": true,
  "visible": false,
  "alwaysOnTop": true,
  "skipTaskbar": true,
  "resizable": false
}
```

### 트레이 (`tray.rs`)

- `TrayIconBuilder`로 메뉴바 아이콘 생성
- 좌클릭 시 창 위치 계산 (아이콘 중앙 기준) + show/hide 토글
- `icon_as_template(true)` — macOS 다크/라이트 모드 자동 대응

### 앱 설정 (`lib.rs`)

- `ActivationPolicy::Accessory` — 독 아이콘 숨김
- `tauri_plugin_notification::init()` 등록
- IPC 커맨드 핸들러 등록

---

## 프론트엔드 구현

- **크기**: 350×500px 고정
- **테마**: 다크 (`#0d0d0d` 배경, `#2563eb` 유저 버블, `#1e1e1e` AI 버블)
- **폰트**: `-apple-system` (시스템 폰트)
- **입력**: Enter 전송 / Shift+Enter 줄바꿈
- **스크롤바**: 숨김 처리

> 현재 `App.tsx`는 scaffold 상태. UI 구현은 Phase 3에서 진행.
