export interface Assistant {
  id: string;
  name: string;
  description: string;
}

// Phase 2: 더미 데이터. Phase 4에서 실제 구현으로 교체.
export const ASSISTANTS: Assistant[] = [
  { id: "scheduler", name: "일정 비서", description: "일정 관리" },
  { id: "links", name: "링크 비서", description: "링크 정리" },
  { id: "general", name: "범용 비서", description: "범용 대화" },
];
