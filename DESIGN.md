# mac-ai-assistant 설계 문서

> macOS 메뉴바 기반 채팅형 AI 비서 앱

---

## 1. 프로젝트 개요

### 컨셉
macOS 상단 메뉴바 아이콘 클릭 시 팝업으로 내려오는 채팅 기반 AI 비서 앱.
로컬 Apple 데이터(Notes, Reminders)와 외부 DB(Supabase)를 AI와 연결하여 개인화된 비서 경험 제공.

### 핵심 기능 요약

| 기능 | 설명 |
|------|------|
| 링크 관리 | Supabase에 저장된 링크/북마크 조회 및 AI 대화에 활용 |
| 일정 관리 | Apple Reminders(미리알림) 조회 → 브리핑 및 질의응답 |
| 메모 기반 대화 | Apple Notes(메모) 내용을 컨텍스트로 1:1 대화 |
| 자동 브리핑 | 외부 트리거 → Gemini API → macOS 알림 센터 |

---

## 2. 기술 스택

| 레이어 | 기술 | 역할 |
|--------|------|------|
| 데스크탑 프레임워크 | **Tauri 2.x** | 네이티브 앱 래퍼, IPC, 시스템 API |
| 프론트엔드 | **React 19 + TypeScript + Vite** | 채팅 UI (WebView) |
| 백엔드 (메인 프로세스) | **Rust** | 트레이 관리, osascript 실행, 알림 |
| AI | **Gemini API** | 대화 생성, 브리핑 (모델은 추후 결정) |
| 외부 DB | **Supabase** | 링크/북마크 데이터 (기존 프로젝트 연동) |
| 로컬 데이터 | **Apple Notes / Reminders** | osascript(AppleScript)로 읽기 |

---

## 3. 아키텍처

### 전체 구조

```
┌──────────────────────────────────────────────────────┐
│                     macOS System                     │
│                                                      │
│  ┌──────────┐    ┌────────────────────────────────┐  │
│  │ Menu Bar │    │          Tauri App              │  │
│  │  Tray ◀──┼────┼──▶  Rust Main Process          │  │
│  └──────────┘    │       ├── Tray Icon Manager     │  │
│                  │       ├── Window Manager         │  │
│  ┌────────────┐  │       ├── osascript Bridge       │  │
│  │Apple Notes │◀─┼───────┤     ├── Notes Reader     │  │
│  │ Reminders  │  │       │     └── Reminders Reader │  │
│  └────────────┘  │       ├── Notification Sender    │  │
│                  │       └── Scheduler (Phase 4)    │  │
│                  │                   │              │  │
│                  │            IPC (invoke)          │  │
│                  │                   │              │  │
│                  │       ┌───────────▼───────────┐  │  │
│                  │       │    WebView (React)     │  │  │
│                  │       │    채팅 UI              │  │  │
│                  │       └───────────────────────┘  │  │
│                  └────────────────────────────────┘  │
│                                                      │
│            ┌──────────────────────────────┐          │
│            │       External Services      │          │
│            │   Gemini API  |  Supabase   │          │
│            └──────────────────────────────┘          │
└──────────────────────────────────────────────────────┘
```

### 데이터 흐름

```
[사용자] 메뉴바 클릭
    → Rust 트레이 이벤트
    → 창 위치 계산 + 표시

[사용자] 채팅 입력
    → React UI
    → Tauri IPC (invoke)
    → Rust: osascript 실행 (Notes / Reminders 조회)
    → 컨텍스트 조립 후 Gemini API 호출 (프론트엔드에서 직접)
    → 스트리밍 응답 → React UI 타이핑 효과

[Phase 4] 외부 트리거 수신
    → 스케줄러 실행
    → Gemini API 브리핑 요청
    → 완료 시 show_notification 커맨드
    → macOS 알림 센터 표시
    → 클릭 시 해당 비서 창 포커스
```

---

## 4. 디렉토리 구조

```
mac-ai-assistant/
├── src/                           # 프론트엔드 (React)
│   ├── components/
│   │   ├── ChatWindow.tsx         # 메인 채팅 컨테이너
│   │   ├── MessageList.tsx        # 스크롤 메시지 목록
│   │   ├── MessageBubble.tsx      # 개별 메시지 버블
│   │   ├── InputBar.tsx           # 텍스트 입력창 + 전송 버튼
│   │   └── AssistantHeader.tsx    # 비서 이름/아바타/액션 버튼
│   ├── hooks/
│   │   ├── useChat.ts             # 채팅 상태 및 Gemini 스트리밍
│   │   └── useAppleData.ts        # Tauri IPC → Notes/Reminders
│   ├── lib/
│   │   ├── assistants.ts          # 비서 정의 (코드로 확장)
│   │   ├── gemini.ts              # Gemini API 스트리밍 클라이언트
│   │   └── supabase.ts            # Supabase REST 조회
│   ├── types/
│   │   └── index.ts               # 공통 타입 정의
│   ├── App.tsx
│   ├── main.tsx
│   └── index.css                  # 다크 테마 글로벌 스타일
│
├── src-tauri/                     # Rust 백엔드
│   ├── src/
│   │   ├── main.rs                # 앱 진입점
│   │   └── lib.rs                 # 트레이, 커맨드, 스케줄러
│   ├── capabilities/
│   │   └── default.json           # Tauri 권한 설정
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── .env                           # 환경 변수 (git 제외)
├── .env.example                   # 환경 변수 템플릿
└── DESIGN.md                      # 이 문서
```

---

## 5. Phase별 구현 가이드

### Phase 1 — 앱 뼈대 (UI & Shell)

**목표**: 메뉴바 아이콘 클릭 시 채팅 팝업 표시

#### Rust 구현 사항

**`tauri.conf.json` 창 설정:**

```json
{
  "label": "main",
  "width": 350,
  "height": 500,
  "decorations": false,
  "transparent": true,
  "visible": false,
  "alwaysOnTop": true,
  "skipTaskbar": true,
  "resizable": false
}
```

**`lib.rs` 핵심 로직:**

```rust
// macOS 독 아이콘 숨기기 (메뉴바 전용 앱)
app.set_activation_policy(tauri::ActivationPolicy::Accessory);

// 트레이 아이콘 + 클릭 이벤트
TrayIconBuilder::new()
    .icon_as_template(true)
    .tooltip("AI 비서")
    .on_tray_icon_event(|tray, event| {
        // 클릭 좌표 기준으로 창 위치 계산
        // 창 show/hide 토글
    })
    .build(app)?;
```

**`Cargo.toml` 의존성:**

```toml
tauri = { version = "2", features = ["tray-icon"] }
tauri-plugin-notification = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

#### 프론트엔드 구현 사항

- **크기**: 350×500px 고정
- **테마**: 다크 (`#0d0d0d` 배경, `#2563eb` 유저 버블, `#1e1e1e` AI 버블)
- **폰트**: `-apple-system` (시스템 폰트)
- **입력**: Enter 전송 / Shift+Enter 줄바꿈
- **스크롤바**: 숨김 처리

---

### Phase 2 — 데이터 레이어 (Data Layer)

**목표**: Apple Notes/Reminders 조회 + Supabase 링크 조회 + IPC 파이프라인

#### Tauri IPC 커맨드 명세

| 커맨드 | 입력 | 반환 | 설명 |
|--------|------|------|------|
| `get_notes` | 없음 | `Result<Vec<Note>, String>` | Apple Notes 전체 조회 |
| `get_reminders` | 없음 | `Result<Vec<Reminder>, String>` | 미완료 미리알림 조회 |
| `show_notification` | `title: String, body: String` | `Result<(), String>` | macOS 알림 발송 |

#### 데이터 타입

```typescript
interface Note {
  name: string;   // 메모 제목
  body: string;   // 본문 (plaintext)
}

interface Reminder {
  name: string;          // 항목명
  due_date?: string;     // 마감일 (있을 경우)
  notes?: string;        // 추가 메모
}
```

#### Apple 데이터 조회 방식

Rust에서 `std::process::Command`로 `osascript -e "..."` 실행.
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

#### Supabase 연동

- SDK 없이 **native fetch**로 REST API 직접 호출 (의존성 최소화)
- 테이블: 링크 테이블 (컬럼: thumbnail, title, description, memo 등 — 연동 시 상세 확인)
- 조회: 최근 10~20개 → Gemini 컨텍스트에 포함

```typescript
// 예시
fetch(`${SUPABASE_URL}/rest/v1/links?select=*&order=created_at.desc&limit=20`, {
  headers: { apikey: SUPABASE_ANON_KEY, Authorization: `Bearer ${SUPABASE_ANON_KEY}` }
})
```

---

### Phase 3 — AI 레이어 (AI Layer)

**목표**: Gemini API 연동, 컨텍스트 기반 스트리밍 대화

#### 비서 구조

비서는 **코드에서 직접 정의** (`src/lib/assistants.ts`). UI에서 생성 불필요.

```typescript
interface Assistant {
  id: string;
  name: string;
  avatar: string;        // emoji
  description: string;
  systemPrompt: string;
}

// 새 비서 추가 시 이 배열에 push
const ASSISTANTS: Assistant[] = [
  {
    id: 'general',
    name: '범용 AI 비서',
    avatar: '🤖',
    systemPrompt: '...',
  },
];
```

#### Gemini API 호출 구조

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
```

**스트리밍 엔드포인트:**
```
POST https://generativelanguage.googleapis.com/v1beta/models/{MODEL}:streamGenerateContent
  ?key={API_KEY}&alt=sse
```

**컨텍스트 주입 전략:**

| 데이터 | 포함 범위 | 제한 |
|--------|-----------|------|
| Notes | 최근 5개 | 본문 200자 요약 |
| Reminders | 미완료 전체 | 날짜 포함 |
| Links | 최근 10개 | title + url + description |

---

### Phase 4 — 스케줄러 & 알림 (Automation)

**목표**: 외부 트리거 → Gemini 브리핑 → macOS 알림 → 클릭 시 채팅창 포커스

#### 동작 흐름

```
외부 트리거 (Gemini 알림 설정)
    ↓
앱 내 이벤트 수신 (딥링크 또는 IPC)
    ↓
Apple 데이터 조회 (Notes + Reminders)
    ↓
Gemini API 브리핑 요청
    ↓
응답 완료
    ↓
show_notification 실행
    ↓
macOS 알림 센터 표시
    ↓
알림 클릭 → 해당 비서 채팅창 포커스
```

**알림 구현 (Rust):**
```rust
use tauri_plugin_notification::NotificationExt;

app.notification()
    .builder()
    .title("AI 비서 브리핑")
    .body("오늘 미완료 미리알림 3건, 링크 5개 저장됨")
    .show()?;
```

> ⚠️ 스케줄 트리거의 구체적인 연동 방식은 Gemini 알림 설정 확인 후 결정

---

## 6. 환경 설정

### .env 파일 구조

```bash
# Gemini API (https://aistudio.google.com/app/apikey)
VITE_GEMINI_API_KEY=your_api_key
VITE_GEMINI_MODEL=gemini-2.0-flash   # 추후 변경 가능

# Supabase (Settings > API)
VITE_SUPABASE_URL=https://your-project.supabase.co
VITE_SUPABASE_ANON_KEY=your_anon_key
```

### macOS 권한 (최초 실행 시 요청)

| 권한 | 위치 |
|------|------|
| 미리알림 접근 | 시스템 설정 > 개인정보 및 보안 > 미리알림 |
| 메모 접근 | 시스템 설정 > 개인정보 및 보안 > 메모 |
| 알림 허용 | 시스템 설정 > 알림 |

---

## 7. 미결 사항 (구현 시 확정 필요)

| 항목 | 현재 상태 | 확정 시점 |
|------|-----------|-----------|
| Gemini 모델 | 미정 | Phase 3 구현 시 |
| Supabase 테이블명 및 컬럼 상세 | 미확인 | Phase 2 Supabase 연동 시 |
| Phase 4 외부 트리거 방식 | Gemini 알림 설정 연동 예정 | Phase 4 착수 시 |

---

## 8. 핵심 설계 결정 요약

| 결정 사항 | 선택 | 이유 |
|-----------|------|------|
| 데스크탑 프레임워크 | Tauri 2.x | 번들 크기 작음, 성능 우수 |
| 비서 추가 방식 | 코드 직접 수정 (`assistants.ts`) | UI 생성 불필요, 개발자 직접 관리 |
| Supabase 연동 | REST API (SDK 없음) | 의존성 최소화 |
| AI 호출 위치 | 프론트엔드 (React) | 스트리밍 처리 용이 |
| Apple 데이터 수집 | Rust osascript | 시스템 권한 안정적 처리 |