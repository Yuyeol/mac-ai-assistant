# Phase 4 — AI 레이어

**목표**: Gemini API 연동, 컨텍스트 기반 스트리밍 대화
**상태**: ⬜ 미시작

---

## 비서 구조

비서는 코드에서 직접 정의 (`src/lib/assistants.ts`). UI에서 생성 불필요.
프롬프트 기반 성격 + 코드로 구현된 기능의 하이브리드.

```typescript
interface Assistant {
  id: string;
  name: string;
  avatar: string;        // emoji 또는 이미지 경로
  description: string;
  systemPrompt: string;  // 성격/말투 정의
  images: {              // 컷 이미지 전환용
    default: string;
    thinking: string;
    happy: string;
    // ...
  };
}
```

---

## Gemini API 호출 구조

```
[시스템 메시지]
비서 시스템 프롬프트
  + 현재 Apple Notes 요약 (최근 5개, 본문 200자 제한)
  + 미완료 Reminders 목록 (날짜 포함)
  + 최근 링크 목록 (title + url + description)

[대화 히스토리]
이전 user/model 메시지

[현재 입력]
사용자 메시지

→ Gemini Streaming Response
→ SSE 파싱 → React 타이핑 효과
→ 응답 감정 분석 → 캐릭터 이미지 전환
```

**스트리밍 엔드포인트:**
```
POST https://generativelanguage.googleapis.com/v1beta/models/{MODEL}:streamGenerateContent
  ?key={API_KEY}&alt=sse
```

---

## 컨텍스트 주입 전략

| 데이터 | 포함 범위 | 제한 |
|--------|-----------|------|
| Notes | 최근 5개 | 본문 200자 요약 |
| Reminders | 미완료 전체 | 날짜 포함 |
| Links | 최근 10개 | title + url + description |

---

## 구현 대상 파일

| 파일 | 내용 |
|------|------|
| `src/lib/gemini.ts` | Gemini API 스트리밍 클라이언트 |
| `src/lib/assistants.ts` | 비서 정의 (더미 → 실제 전환) |
| `src/hooks/useChat.ts` | 채팅 상태 및 Gemini 스트리밍 훅 |
| `src/hooks/useAppleData.ts` | Tauri IPC → Notes/Reminders 조회 |
