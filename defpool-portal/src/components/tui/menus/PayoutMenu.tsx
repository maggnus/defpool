import { useState } from "react";
import TuiModal from "../TuiModal";
import TuiInput from "../TuiInput";
import TuiDropdown from "../TuiDropdown";
import TuiCheckbox from "../TuiCheckbox";
import TuiButton from "../TuiButton";
import TuiRow from "../TuiRow";

interface PayoutMenuProps {
  isOpen: boolean;
  onClose: () => void;
}

const PayoutMenu = ({ isOpen, onClose }: PayoutMenuProps) => {
  const [minPayout, setMinPayout] = useState("0.001");
  const [payoutCoin, setPayoutCoin] = useState("btc");
  const [autoConvert, setAutoConvert] = useState(true);
  const [schedule, setSchedule] = useState("daily");

  return (
    <TuiModal title="PAYOUT SETTINGS" isOpen={isOpen} onClose={onClose} width="max-w-lg">
      <div className="space-y-3">
        <div className="border-b border-border pb-2 text-muted-foreground text-xs">
          CURRENT BALANCE
        </div>
        <TuiRow label="UNPAID" value="0.0847 BTC" variant="up" />
        <TuiRow label="ESTIMATED" value="$4,235.00" />
        <TuiRow label="NEXT PAYOUT" value="2h 14m" />

        <div className="border-b border-border pb-2 pt-4 text-muted-foreground text-xs">
          PAYOUT OPTIONS
        </div>
        <TuiInput
          label="MIN PAYOUT"
          value={minPayout}
          onChange={(e) => setMinPayout(e.target.value)}
          type="number"
          step="0.001"
        />
        <TuiDropdown
          label="PAYOUT COIN"
          value={payoutCoin}
          onChange={setPayoutCoin}
          options={[
            { value: "btc", label: "Bitcoin (BTC)" },
            { value: "eth", label: "Ethereum (ETH)" },
            { value: "usdt", label: "Tether (USDT)" },
            { value: "original", label: "Original Coin" },
          ]}
        />
        <TuiDropdown
          label="SCHEDULE"
          value={schedule}
          onChange={setSchedule}
          options={[
            { value: "threshold", label: "When threshold reached" },
            { value: "daily", label: "Daily" },
            { value: "weekly", label: "Weekly" },
          ]}
        />
        <TuiCheckbox
          label="Auto-convert to payout coin"
          checked={autoConvert}
          onChange={setAutoConvert}
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

export default PayoutMenu;
