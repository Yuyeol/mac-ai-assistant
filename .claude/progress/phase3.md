# Phase 3 — 앱 창 UI 껍데기

**목표**: 앱 창의 몰입형 채팅 레이아웃 완성 (기능 연결 없음)
**상태**: ⬜ 미시작

> UI 스펙: @.claude/spec/frontend/ui/app-window.md

---

## 구현 범위

- 캐릭터 이미지 영역 (더미 이미지)
- 말풍선 채팅 레이아웃
- 비서 선택 사이드바 또는 탭
- 입력바 (팝업과 동일 컴포넌트 재사용)
- 이미지 컷 전환 로직 틀 (실제 트리거는 Phase 4)

## 구현 대상 파일

| 파일 | 내용 |
|------|------|
| `src/components/app/AppWindow.tsx` | 앱 창 루트 컴포넌트 |
| `src/components/app/CharacterView.tsx` | 캐릭터 이미지 영역 |
| `src/components/app/ChatBubble.tsx` | 말풍선 메시지 |
| `src/components/app/MessageList.tsx` | 채팅 스크롤 영역 |
| `src/components/app/AssistantSidebar.tsx` | 비서 전환 사이드바 |

## 미결 사항

- Live2D 라이브러리 적용 여부 결정 필요
- 캐릭터 이미지 컷 수 및 감정 상태 정의 필요
