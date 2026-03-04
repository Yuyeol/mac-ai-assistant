# Architecture Overview

> macOS 메뉴바 기반 채팅형 AI 비서 앱

---

## 개요

macOS 상단 메뉴바 아이콘 클릭 시 팝업으로 내려오는 채팅 기반 AI 비서.
로컬 Apple 데이터(Notes, Reminders)와 외부 DB(Supabase)를 AI와 연결하여 개인화된 비서 경험 제공.

| 기능           | 설명                                                 |
| -------------- | ---------------------------------------------------- |
| 링크 관리      | Supabase에 저장된 링크/북마크 조회 및 AI 대화에 활용 |
| 일정 관리      | Apple Reminders 조회 → 브리핑 및 질의응답            |
| 메모 기반 대화 | Apple Notes 내용을 컨텍스트로 1:1 대화               |
| 자동 브리핑    | 외부 트리거 → Gemini API → macOS 알림 센터           |

---

## 기술 스택

| 레이어              | 기술                         | 역할                              |
| ------------------- | ---------------------------- | --------------------------------- |
| 데스크탑 프레임워크 | Tauri 2.x                    | 네이티브 앱 래퍼, IPC, 시스템 API |
| 프론트엔드          | React 19 + TypeScript + Vite | 채팅 UI (WebView)                 |
| 스타일링            | Tailwind CSS v4              | UI 스타일링 (`@tailwindcss/vite`) |
| 백엔드              | Rust                         | 트레이 관리, osascript 실행, 알림 |
| AI                  | Gemini API                   | 대화 생성, 브리핑                 |
| 외부 DB             | Supabase                     | 링크/북마크 데이터                |
| 로컬 데이터         | Apple Notes / Reminders      | osascript(AppleScript)로 읽기     |

---

## 아키텍처

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

---
