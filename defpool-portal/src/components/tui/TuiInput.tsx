import { InputHTMLAttributes } from "react";

interface TuiInputProps extends InputHTMLAttributes<HTMLInputElement> {
  label: string;
}

const TuiInput = ({ label, ...props }: TuiInputProps) => {
  return (
    <div className="flex items-center gap-2 py-1">
      <span className="tui-label min-w-[120px]">{label}</span>
      <input
        {...props}
        className="flex-1 bg-secondary border border-border px-2 py-1 text-foreground focus:outline-none focus:border-foreground"
      />
    </div>
  );
};

export default TuiInput;
