# CLAUDE.md

macOS 메뉴바 팝업 AI 비서 앱 (Tauri + React + Gemini).

## 개발 명령어

```bash
pnpm tauri dev   # Vite(포트 1420) + Rust 동시 실행
pnpm tauri build # 프로덕션 빌드
pnpm dev         # 프론트엔드만 (UI 확인용)
```

환경 변수: `.env.example` 참고 (`VITE_GEMINI_API_KEY`, `VITE_SUPABASE_URL` 등).

## 문서

- 전체 아키텍처 및 데이터 흐름: @.claude/architecture/overview.md
- React 프론트엔드 UI 스펙: @.claude/spec/frontend/App.md
- Rust 데이터 모델: @.claude/spec/backend/models.md
- Notes / Reminders IPC 스펙: @.claude/spec/backend/data/notes.md, @.claude/spec/backend/data/reminders.md
- Notification 액션 스펙: @.claude/spec/backend/actions/notification.md
- 팝업 UI 스펙: @.claude/spec/frontend/ui/popup.md
- 앱 창 UI 스펙: @.claude/spec/frontend/ui/app-window.md
- Phase별 구현 진행 상황: @.claude/progress/overview.md
