# Notes Spec

## Overview

Apple Notes 앱의 전체 노트를 읽어 반환.

---

## API

### `get_notes`

| 항목 | 내용 |
|------|------|
| 파일 | `commands/notes.rs` |
| 방식 | `osascript` 실행 → stdout 파싱 |
| 파라미터 | 없음 |
| 반환 | `Vec<Note>` |
| 에러 | `String` (osascript 실행 실패 시) |

```ts
const notes = await invoke<Note[]>("get_notes");
```

---

## 파싱 규칙

osascript stdout 구분자:

| 구분자 | 용도 |
|--------|------|
| `###` | 노트 간 구분 |
| `\|\|\|` | 제목 / 본문 구분 |

> Apple Notes 접근은 **macOS Privacy 권한** 필요. 첫 실행 시 사용자 동의.
