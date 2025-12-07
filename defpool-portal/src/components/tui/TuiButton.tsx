import { ButtonHTMLAttributes } from "react";

interface TuiButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "default" | "primary" | "danger";
}

const TuiButton = ({ children, variant = "default", className = "", ...props }: TuiButtonProps) => {
  const variantClasses = {
    default: "border-border hover:bg-secondary",
    primary: "border-foreground bg-foreground text-background hover:bg-muted-foreground",
    danger: "border-destructive text-destructive hover:bg-destructive hover:text-destructive-foreground",
  }[variant];

  return (
    <button
      {...props}
      className={`px-4 py-1 border text-sm transition-colors ${variantClasses} ${className}`}
    >
      {children}
    </button>
  );
};

export default TuiButton;
