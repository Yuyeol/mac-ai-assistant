# Reminders Spec

## Overview

Apple Reminders 앱의 **미완료** 항목 전체를 읽어 반환. 모든 목록(List) 순회.

---

## API

### `get_reminders`

| 항목 | 내용 |
|------|------|
| 파일 | `commands/reminders.rs` |
| 방식 | `osascript` 실행 → stdout 파싱 |
| 파라미터 | 없음 |
| 반환 | `Vec<Reminder>` |
| 에러 | `String` (osascript 실행 실패 시) |

```ts
const reminders = await invoke<Reminder[]>("get_reminders");
```

---

## 파싱 규칙

osascript stdout 구분자:

| 구분자 | 용도 |
|--------|------|
| `###` | 항목 간 구분 |
| `\|\|\|` | 제목 / 마감일 / 메모 구분 |

- `due_date`, `notes` 는 값이 없으면 빈 문자열 → `None` 처리
- 완료된 항목(`completed = true`)은 제외

> Apple Reminders 접근은 **macOS Privacy 권한** 필요. 첫 실행 시 사용자 동의.
