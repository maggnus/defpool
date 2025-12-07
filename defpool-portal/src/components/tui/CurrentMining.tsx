import { useCurrentTarget, useCurrentTargetName } from "@/hooks/use-defpool-api";

const CurrentMining = () => {
  const { data: currentTarget, isLoading: targetLoading } = useCurrentTarget();
  const { data: currentTargetName } = useCurrentTargetName();

  // Helper function to get algorithm display name
  const getAlgorithmDisplay = (coin: string): string => {
    switch (coin) {
      case "XMR":
        return "RandomX";
      case "LTC":
      case "DOGE":
        return "Scrypt";
      default:
        return "Unknown";
    }
  };

  if (targetLoading) {
    return (
      <div className="tui-window h-full">
        <div className="tui-title">[ CURRENT MINING ]</div>
        <div className="tui-content flex items-center justify-center">
          <span className="text-muted-foreground">Loading...</span>
        </div>
      </div>
    );
  }

  if (!currentTarget) {
    return (
      <div className="tui-window h-full">
        <div className="tui-title">[ CURRENT MINING ]</div>
        <div className="tui-content flex items-center justify-center">
          <span className="text-red-500">No mining target</span>
        </div>
      </div>
    );
  }

  return (
    <div className="tui-window h-full">
      <div className="tui-title">[ CURRENT MINING ]</div>
      <div className="tui-content space-y-1">
        <div className="tui-row">
          <span className="tui-label">POOL</span>
          <span className="tui-value">{currentTargetName || "Unknown"}</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">ADDRESS</span>
          <span className="tui-value">{currentTarget.address}</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">PROTOCOL</span>
          <span className="tui-value">{currentTarget.protocol.toUpperCase()}</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">STATUS</span>
          <span className="tui-value-up">ACTIVE</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">UPTIME</span>
          <span className="tui-value">System Online</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">LAST UPDATE</span>
          <span className="tui-value">{new Date().toLocaleTimeString()}</span>
        </div>
      </div>
    </div>
  );
};

export default CurrentMining;
