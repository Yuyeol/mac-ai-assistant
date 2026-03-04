# Phase 6 — 자동화 (Automation)

**목표**: 외부 트리거 → Gemini 브리핑 → macOS 알림 → 클릭 시 앱 창 포커스
**상태**: ⬜ 미시작

---

## 동작 흐름

```
외부 트리거 (Gemini 알림 설정)
    ↓
앱 내 이벤트 수신 (딥링크 또는 IPC)
    ↓
Apple 데이터 조회 (Notes + Reminders)
    ↓
Gemini API 브리핑 요청
    ↓
응답 완료
    ↓
show_notification 실행
    ↓
macOS 알림 센터 표시
    ↓
알림 클릭 → 앱 창 포커스 (해당 비서 선택 상태)
```

---

## 알림 구현 (Rust)

```rust
use tauri_plugin_notification::NotificationExt;

app.notification()
    .builder()
    .title("AI 비서 브리핑")
    .body("오늘 미완료 미리알림 3건, 링크 5개 저장됨")
    .show()?;
```

---

## 미결 사항

- 외부 트리거 방식 미확정 (Gemini 알림 설정 확인 후 결정)
- 딥링크 vs IPC 수신 방식 검토 필요
- 알림 클릭 → 창 포커스 연결 구현 방법 미정
