import { ReactNode } from "react";

interface TuiRowProps {
  label: string;
  value: ReactNode;
  variant?: "default" | "up" | "down" | "warn";
}

const TuiRow = ({ label, value, variant = "default" }: TuiRowProps) => {
  const valueClass = {
    default: "tui-value",
    up: "tui-value-up",
    down: "tui-value-down",
    warn: "tui-value-warn",
  }[variant];

  return (
    <div className="tui-row">
      <span className="tui-label">{label}</span>
      <span className={valueClass}>{value}</span>
    </div>
  );
};

export default TuiRow;
