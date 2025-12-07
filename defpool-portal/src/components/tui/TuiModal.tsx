import { ReactNode, useEffect } from "react";

interface TuiModalProps {
  title: string;
  isOpen: boolean;
  onClose: () => void;
  children: ReactNode;
  width?: string;
}

const TuiModal = ({ title, isOpen, onClose, children, width = "max-w-lg" }: TuiModalProps) => {
  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    if (isOpen) {
      document.addEventListener("keydown", handleEscape);
    }
    return () => document.removeEventListener("keydown", handleEscape);
  }, [isOpen, onClose]);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      <div className="absolute inset-0 bg-background/80" onClick={onClose} />
      <div className={`relative tui-window ${width} w-full mx-4`}>
        <div className="tui-title flex justify-between items-center">
          <span>[ {title} ]</span>
          <button onClick={onClose} className="text-muted-foreground hover:text-foreground">
            [X]
          </button>
        </div>
        <div className="tui-content">{children}</div>
      </div>
    </div>
  );
};

export default TuiModal;
