import { ReactNode } from "react";

interface TuiWindowProps {
  title: string;
  children: ReactNode;
  className?: string;
}

const TuiWindow = ({ title, children, className = "" }: TuiWindowProps) => {
  return (
    <div className={`tui-window h-full ${className}`}>
      <div className="tui-title">[ {title} ]</div>
      <div className="tui-content">{children}</div>
    </div>
  );
};

export default TuiWindow;
