import { useState, useRef, useEffect } from "react";

interface TuiDropdownProps {
  label: string;
  value: string;
  onChange: (value: string) => void;
  options: { value: string; label: string }[];
}

const TuiDropdown = ({ label, value, onChange, options }: TuiDropdownProps) => {
  const [isOpen, setIsOpen] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

  const selectedLabel = options.find((opt) => opt.value === value)?.label || value;

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  return (
    <div className="flex items-center gap-2 py-1" ref={dropdownRef}>
      <span className="tui-label min-w-[120px]">{label}</span>
      <div className="relative flex-1">
        <button
          type="button"
          onClick={() => setIsOpen(!isOpen)}
          className="w-full text-left bg-secondary border border-border px-2 py-1 text-foreground hover:border-foreground focus:outline-none focus:border-foreground flex justify-between items-center"
        >
          <span>{selectedLabel}</span>
          <span className="text-muted-foreground">{isOpen ? "[-]" : "[+]"}</span>
        </button>
        {isOpen && (
          <div className="absolute top-full left-0 right-0 z-50 bg-card border border-border mt-0">
            {options.map((opt) => (
              <button
                key={opt.value}
                type="button"
                onClick={() => {
                  onChange(opt.value);
                  setIsOpen(false);
                }}
                className={`w-full text-left px-2 py-1 hover:bg-secondary ${
                  opt.value === value ? "text-foreground bg-secondary" : "text-muted-foreground"
                }`}
              >
                {opt.value === value && "> "}
                {opt.label}
              </button>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default TuiDropdown;
