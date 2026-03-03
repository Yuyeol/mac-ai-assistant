# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 개발 명령어

```bash
# 개발 서버 실행 (Vite + Rust 동시 빌드)
pnpm tauri dev

# 프로덕션 빌드
pnpm tauri build

# 프론트엔드만 개발 (Tauri 없이 UI 확인용)
pnpm dev
```

Vite는 포트 **1420** 고정 (변경 불가, Tauri 요구사항).
Rust 변경사항은 `tauri dev` 실행 중 자동 재컴파일됨.

## 환경 변수

실행 전 프로젝트 루트에 `.env` 파일 필요 (`.env.example` 참고):

```
VITE_GEMINI_API_KEY=   # Google AI Studio에서 발급
VITE_GEMINI_MODEL=     # 예: gemini-2.0-flash
VITE_SUPABASE_URL=     # Supabase 프로젝트 URL
VITE_SUPABASE_ANON_KEY=
```

## 아키텍처

macOS 전용 메뉴바 팝업 앱. 두 개의 레이어로 구성:

**Rust 메인 프로세스 (`src-tauri/src/lib.rs`)**
- 시스템 트레이 아이콘 생성 및 클릭 이벤트로 WebView 창 토글
- `osascript`로 Apple Notes / Reminders 데이터 읽기
- `tauri-plugin-notification`으로 macOS 알림 발송
- macOS 독 아이콘 숨김: `ActivationPolicy::Accessory`

**React 프론트엔드 (`src/`)**
- Tauri IPC(`invoke`)로 Rust 커맨드 호출 → Apple 데이터 수집
- Gemini API 직접 호출 (프론트엔드에서 SSE 스트리밍)
- Supabase REST API 직접 호출 (SDK 미사용)

### 주요 Tauri 커맨드 (IPC)

| 커맨드 | 반환 | 용도 |
|--------|------|------|
| `get_notes` | `Vec<Note>` | Apple Notes 전체 |
| `get_reminders` | `Vec<Reminder>` | 미완료 미리알림 |
| `show_notification` | `()` | macOS 알림 발송 |

### 비서(Assistant) 확장 방법

`src/lib/assistants.ts`의 `ASSISTANTS` 배열에 항목 추가. UI에서 생성하는 기능 없음.

## 설계 문서

전체 아키텍처, 데이터 흐름, Phase별 구현 가이드는 **`DESIGN.md`** 참고.
