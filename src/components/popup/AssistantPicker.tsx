import { useState, useEffect, useRef } from "react";
import { Assistant } from "../../lib/assistants";

interface Props {
  assistants: Assistant[];
  selectedId: string;
  onSelect: (id: string) => void;
}

export function AssistantPicker({ assistants, selectedId, onSelect }: Props) {
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);
  const selected = assistants.find((a) => a.id === selectedId) ?? assistants[0];

  useEffect(() => {
    function handleClick(e: MouseEvent) {
      if (ref.current && !ref.current.contains(e.target as Node)) {
        setOpen(false);
      }
    }
    document.addEventListener("mousedown", handleClick);
    return () => document.removeEventListener("mousedown", handleClick);
  }, []);

  return (
    <div ref={ref} className="relative">
      <button
        onClick={() => setOpen(!open)}
        className="bg-transparent border-none text-[#8e8e93] hover:text-[#ebebf5] transition-colors cursor-pointer text-[13px] font-[inherit] flex items-center gap-1 px-1 py-0.5 rounded"
      >
        {selected.name}
        <span className="text-[10px] rotate-90 inline-block">›</span>
      </button>

      {open && (
        <div className="absolute bottom-full mb-1.5 right-0 bg-[#2c2c2e] rounded-lg overflow-hidden min-w-[160px] shadow-[0_4px_20px_rgba(0,0,0,0.5)] z-10">
          {assistants.map((a) => (
            <div
              key={a.id}
              onClick={() => {
                onSelect(a.id);
                setOpen(false);
              }}
              className={`px-3.5 py-2.5 cursor-pointer flex justify-between items-center text-[#ebebf5] text-[13px] transition-colors hover:bg-[#3a3a3c] ${a.id === selectedId ? "bg-[#3a3a3c]" : ""}`}
            >
              <span>{a.name}</span>
              <span className="text-[#636366] text-[11px]">{a.description}</span>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
