# Data Models Spec

Rust ↔ React 직렬화 구조체. `serde` 기반 JSON 변환.

---

## Note

Apple Notes 노트 1건.

```rust
pub struct Note {
    pub name: String,  // 노트 제목
    pub body: String,  // 노트 본문 (plaintext)
}
```

```ts
// TypeScript 대응 타입
interface Note {
  name: string;
  body: string;
}
```

---

## Reminder

Apple Reminders 미리알림 1건.

```rust
pub struct Reminder {
    pub name: String,             // 미리알림 제목
    pub due_date: Option<String>, // 마감일 문자열 (없으면 null)
    pub notes: Option<String>,    // 메모 (없으면 null)
}
```

```ts
// TypeScript 대응 타입
interface Reminder {
  name: string;
  due_date: string | null;
  notes: string | null;
}
```
