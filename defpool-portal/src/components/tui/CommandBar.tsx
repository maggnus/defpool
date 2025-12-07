interface CommandBarProps {
  onOpenSettings: () => void;
  onOpenWallets: () => void;
  onOpenPayout: () => void;
  onRefresh: () => void;
  onOpenHelp: () => void;
}

const CommandBar = ({
  onOpenSettings,
  onOpenWallets,
  onOpenPayout,
  onRefresh,
  onOpenHelp,
}: CommandBarProps) => {
  return (
    <div className="flex items-center px-2 py-1 border-t border-border bg-secondary text-xs gap-4">
      <span className="text-muted-foreground">COMMANDS:</span>
      <button onClick={onOpenSettings} className="hover:text-foreground">
        <span className="text-foreground">[S]</span>
        <span className="text-muted-foreground">ettings</span>
      </button>
      <button onClick={onOpenWallets} className="hover:text-foreground">
        <span className="text-foreground">[W]</span>
        <span className="text-muted-foreground">allets</span>
      </button>
      <button onClick={onOpenPayout} className="hover:text-foreground">
        <span className="text-foreground">[P]</span>
        <span className="text-muted-foreground">ayout</span>
      </button>
      <button onClick={onRefresh} className="hover:text-foreground">
        <span className="text-foreground">[R]</span>
        <span className="text-muted-foreground">efresh</span>
      </button>
      <button onClick={onOpenHelp} className="hover:text-foreground">
        <span className="text-foreground">[H]</span>
        <span className="text-muted-foreground">elp</span>
      </button>
      <div className="flex-1" />
      <span className="text-muted-foreground">
        NEXT PAYOUT IN: <span className="text-foreground">2h 14m</span>
      </span>
    </div>
  );
};

export default CommandBar;
