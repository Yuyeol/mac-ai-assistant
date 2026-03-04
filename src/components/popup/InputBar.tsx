import { useState } from "react";
import { AssistantPicker } from "./AssistantPicker";
import { ASSISTANTS } from "../../lib/assistants";

export function InputBar() {
  const [value, setValue] = useState("");
  const [selectedId, setSelectedId] = useState(ASSISTANTS[0].id);

  return (
    <div className="bg-[#1c1c1e] rounded-xl px-3.5 pt-3 pb-2.5 m-2 flex flex-col gap-2">
      <textarea
        value={value}
        onChange={(e) => setValue(e.target.value)}
        placeholder="답글..."
        rows={1}
        className="bg-transparent border-none outline-none text-[#ebebf5] text-[15px] resize-none w-full placeholder:text-[#636366] font-[inherit] leading-snug"
        onKeyDown={(e) => {
          if (e.key === "Enter" && !e.shiftKey) {
            e.preventDefault();
            // Phase 4: Gemini 호출
          }
        }}
      />
      <div className="flex items-center justify-between">
        <button className="text-[#636366] hover:text-[#ebebf5] transition-colors cursor-pointer bg-transparent border-none text-lg px-1 py-0.5 rounded">
          +
        </button>
        <div className="flex items-center gap-1.5">
          <AssistantPicker
            assistants={ASSISTANTS}
            selectedId={selectedId}
            onSelect={setSelectedId}
          />
          <button className="text-[#636366] hover:text-[#ebebf5] transition-colors cursor-pointer bg-transparent border-none flex items-center px-1 py-0.5 rounded">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.8">
              <path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z"/>
              <path d="M19 10v2a7 7 0 0 1-14 0v-2"/>
              <line x1="12" y1="19" x2="12" y2="23"/>
              <line x1="8" y1="23" x2="16" y2="23"/>
            </svg>
          </button>
        </div>
      </div>
    </div>
  );
}
