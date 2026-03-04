# Phase 2 — 팝업 UI 껍데기

**목표**: 메뉴바 팝업의 시각적 레이아웃 완성 (기능 연결 없음)
**상태**: ✅ 완료

> UI 스펙: @.claude/spec/frontend/ui/popup.md

---

## 구현 내용

### 창 설정
- 크기: `480 × 110px`
- 위치: 화면 하단 중앙 고정 (Retina scale factor 반영)
- 닫기: 포커스 잃으면 자동 숨김 (`WindowEvent::Focused(false)`)

### 파일 목록

| 파일 | 내용 |
|------|------|
| `src/App.tsx` | 팝업 진입점, `InputBar` 렌더링 |
| `src/App.css` | `@import "tailwindcss"` + 전역 base 스타일 |
| `src/components/popup/InputBar.tsx` | 입력바 전체 컨테이너 |
| `src/components/popup/AssistantPicker.tsx` | 비서 선택 드롭다운 (위로 펼쳐짐) |
| `src/lib/assistants.ts` | 비서 목록 더미 정의 (3개) |
| `src-tauri/src/tray.rs` | 하단 중앙 좌표 계산 로직 |
| `src-tauri/tauri.conf.json` | 창 크기 `480×110` 설정 |

### 더미 비서 목록

| id | name | description |
|----|------|-------------|
| `scheduler` | 일정 비서 | 일정 관리 |
| `links` | 링크 비서 | 링크 정리 |
| `general` | 범용 비서 | 범용 대화 |

### 스타일링

Tailwind CSS v4 (`@tailwindcss/vite`) 사용.
전송/음성 버튼은 자리만 존재, Phase 4에서 기능 연결.
