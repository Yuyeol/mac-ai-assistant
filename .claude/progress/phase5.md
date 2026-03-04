# Phase 5 — 데이터 레이어

**목표**: Apple Notes/Reminders 조회 + Supabase 링크 조회 + IPC 파이프라인
**상태**: ⬜ 미시작

---

## Apple IPC (Rust 커맨드 — Phase 1에서 이미 구현)

| 커맨드 | 입력 | 반환 |
|--------|------|------|
| `get_notes` | 없음 | `Result<Vec<Note>, String>` |
| `get_reminders` | 없음 | `Result<Vec<Reminder>, String>` |
| `show_notification` | `title`, `body` | `Result<(), String>` |

Phase 4의 `useAppleData.ts` hook과 연결.

---

## Supabase 연동

- SDK 없이 native fetch로 REST API 직접 호출
- 테이블명 및 컬럼 구조 확인 필요 (미결 사항)
- 조회: 최근 10~20개 → Gemini 컨텍스트에 포함

```typescript
fetch(`${SUPABASE_URL}/rest/v1/links?select=*&order=created_at.desc&limit=20`, {
  headers: {
    apikey: SUPABASE_ANON_KEY,
    Authorization: `Bearer ${SUPABASE_ANON_KEY}`
  }
})
```

---

## 구현 대상 파일

| 파일 | 내용 |
|------|------|
| `src/lib/supabase.ts` | Supabase REST 조회 |
| `src/hooks/useAppleData.ts` | Tauri IPC → Notes/Reminders 훅 (Phase 4와 공유) |
