# Phase 2 — 데이터 레이어 (Data Layer)

**목표**: Apple Notes/Reminders 조회 + Supabase 링크 조회 + IPC 파이프라인
**상태**: 🔄 진행 중 (Apple IPC 완료 / Supabase 미연동)

---

## 완료

### Tauri IPC 커맨드

| 커맨드 | 입력 | 반환 |
|--------|------|------|
| `get_notes` | 없음 | `Result<Vec<Note>, String>` |
| `get_reminders` | 없음 | `Result<Vec<Reminder>, String>` |
| `show_notification` | `title`, `body` | `Result<(), String>` |

### Apple 데이터 조회 방식

Rust `std::process::Command`로 `osascript -e "..."` 실행.
출력은 구분자(`|||`, `###`)로 파싱.

**Notes AppleScript:**
```applescript
tell application "Notes"
    repeat with n in notes
        -- name, plaintext 순서로 추출
    end repeat
end tell
```

**Reminders AppleScript:**
```applescript
tell application "Reminders"
    repeat with aList in lists
        repeat with r in reminders of aList
            if not completed of r then
                -- name, due date, body 추출
            end if
        end repeat
    end repeat
end tell
```

---

## 미완료

### Supabase 연동

- SDK 없이 native fetch로 REST API 직접 호출
- 테이블명 및 컬럼 구조 확인 필요 (미결 사항)
- 조회: 최근 10~20개 → Gemini 컨텍스트에 포함 예정

```typescript
fetch(`${SUPABASE_URL}/rest/v1/links?select=*&order=created_at.desc&limit=20`, {
  headers: {
    apikey: SUPABASE_ANON_KEY,
    Authorization: `Bearer ${SUPABASE_ANON_KEY}`
  }
})
```

### 프론트엔드 hooks

- `useAppleData.ts` — Tauri IPC → Notes/Reminders 조회
- `supabase.ts` — Supabase REST 조회
