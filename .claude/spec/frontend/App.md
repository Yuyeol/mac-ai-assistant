# Frontend Spec

## Overview

macOS 메뉴바 팝업 창에 렌더링되는 UI 레이어.
Tauri IPC(`invoke`)로 Rust 커맨드를 호출하고,
Gemini API(SSE 스트리밍)와 Supabase REST API를 직접 호출.

> 현재 `App.tsx`는 초기 scaffold 상태. 실제 기능 UI 미구현.

---

## Data Flow

```
[메뉴바 클릭]
    │
    ▼
[Tauri WebView 창 표시]
    │
    ├─ invoke("get_notes")                    → Apple Notes
    ├─ invoke("get_reminders")                → Apple Reminders
    └─ invoke("show_notification", {...})     → macOS 알림 센터

[AI 응답]
    └─ fetch(Gemini API)  ← SSE 스트리밍, 프론트엔드 직접 호출

[데이터 저장/조회]
    └─ fetch(Supabase REST API)  ← SDK 미사용, 직접 호출
```

---

## Configuration

`.env` 파일 (프로젝트 루트):

| 변수 | 용도 |
|------|------|
| `VITE_GEMINI_API_KEY` | Google AI Studio API 키 |
| `VITE_GEMINI_MODEL` | 모델명 (예: `gemini-2.0-flash`) |
| `VITE_SUPABASE_URL` | Supabase 프로젝트 URL |
| `VITE_SUPABASE_ANON_KEY` | Supabase anon 키 |

> Vite 개발 서버 포트 **1420** 고정 (Tauri 요구사항).
