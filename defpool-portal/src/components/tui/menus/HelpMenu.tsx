import TuiModal from "../TuiModal";
import TuiButton from "../TuiButton";

interface HelpMenuProps {
  isOpen: boolean;
  onClose: () => void;
}

const HelpMenu = ({ isOpen, onClose }: HelpMenuProps) => {
  return (
    <TuiModal title="HELP" isOpen={isOpen} onClose={onClose} width="max-w-xl">
      <div className="space-y-4">
        <div>
          <div className="text-foreground mb-2">KEYBOARD SHORTCUTS</div>
          <div className="space-y-1 text-xs">
            <div className="flex justify-between">
              <span className="text-muted-foreground">Open Settings</span>
              <span className="text-foreground">[S]</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Manage Wallets</span>
              <span className="text-foreground">[W]</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Payout Settings</span>
              <span className="text-foreground">[P]</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Refresh Data</span>
              <span className="text-foreground">[R]</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Show Help</span>
              <span className="text-foreground">[H]</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Close Modal</span>
              <span className="text-foreground">[ESC]</span>
            </div>
          </div>
        </div>

        <div className="border-t border-border pt-4">
          <div className="text-foreground mb-2">GETTING STARTED</div>
          <div className="text-muted-foreground text-xs space-y-2">
            <p>1. Configure your wallet addresses in [W]allets</p>
            <p>2. Set your mining preferences in [S]ettings</p>
            <p>3. Configure payout options in [P]ayout</p>
            <p>4. Monitor your mining stats in real-time</p>
          </div>
        </div>

        <div className="border-t border-border pt-4">
          <div className="text-foreground mb-2">SUPPORT</div>
          <div className="text-muted-foreground text-xs space-y-1">
            <p>Discord: discord.gg/automine</p>
            <p>Email: support@automine.io</p>
            <p>Docs: docs.automine.io</p>
          </div>
        </div>

        <div className="flex gap-2 pt-4 justify-end">
          <TuiButton variant="primary" onClick={onClose}>
            Close
          </TuiButton>
        </div>
      </div>
    </TuiModal>
  );
};

export default HelpMenu;
