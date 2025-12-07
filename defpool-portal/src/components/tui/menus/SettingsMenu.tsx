import { useState } from "react";
import TuiModal from "../TuiModal";
import TuiInput from "../TuiInput";
import TuiDropdown from "../TuiDropdown";
import TuiCheckbox from "../TuiCheckbox";
import TuiButton from "../TuiButton";

interface SettingsMenuProps {
  isOpen: boolean;
  onClose: () => void;
}

const SettingsMenu = ({ isOpen, onClose }: SettingsMenuProps) => {
  const [poolUrl, setPoolUrl] = useState("stratum+tcp://pool.automine.io:3333");
  const [workerName, setWorkerName] = useState("worker1");
  const [intensity, setIntensity] = useState("high");
  const [autoSwitch, setAutoSwitch] = useState(true);
  const [notifications, setNotifications] = useState(true);
  const [tempLimit, setTempLimit] = useState("85");

  return (
    <TuiModal title="SETTINGS" isOpen={isOpen} onClose={onClose} width="max-w-xl">
      <div className="space-y-3">
        <div className="border-b border-border pb-2 text-muted-foreground text-xs">
          CONNECTION
        </div>
        <TuiInput
          label="POOL URL"
          value={poolUrl}
          onChange={(e) => setPoolUrl(e.target.value)}
        />
        <TuiInput
          label="WORKER NAME"
          value={workerName}
          onChange={(e) => setWorkerName(e.target.value)}
        />

        <div className="border-b border-border pb-2 pt-4 text-muted-foreground text-xs">
          MINING
        </div>
        <TuiDropdown
          label="INTENSITY"
          value={intensity}
          onChange={setIntensity}
          options={[
            { value: "low", label: "Low" },
            { value: "medium", label: "Medium" },
            { value: "high", label: "High" },
          ]}
        />
        <TuiInput
          label="TEMP LIMIT"
          value={tempLimit}
          onChange={(e) => setTempLimit(e.target.value)}
          type="number"
          placeholder="°C"
        />

        <div className="border-b border-border pb-2 pt-4 text-muted-foreground text-xs">
          OPTIONS
        </div>
        <TuiCheckbox
          label="Enable auto-switching"
          checked={autoSwitch}
          onChange={setAutoSwitch}
        />
        <TuiCheckbox
          label="Enable notifications"
          checked={notifications}
          onChange={setNotifications}
        />

        <div className="flex gap-2 pt-4 justify-end">
          <TuiButton onClick={onClose}>Cancel</TuiButton>
          <TuiButton variant="primary" onClick={onClose}>
            Save
          </TuiButton>
        </div>
      </div>
    </TuiModal>
  );
};

export default SettingsMenu;
