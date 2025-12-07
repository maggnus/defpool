interface TuiCheckboxProps {
  label: string;
  checked: boolean;
  onChange: (checked: boolean) => void;
}

const TuiCheckbox = ({ label, checked, onChange }: TuiCheckboxProps) => {
  return (
    <label className="flex items-center gap-2 py-1 cursor-pointer">
      <span className="text-foreground">[{checked ? "X" : " "}]</span>
      <span className="tui-label">{label}</span>
      <input
        type="checkbox"
        checked={checked}
        onChange={(e) => onChange(e.target.checked)}
        className="sr-only"
      />
    </label>
  );
};

export default TuiCheckbox;
