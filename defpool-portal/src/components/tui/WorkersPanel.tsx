import { useState } from "react";
import { useMinerWorkers } from "@/hooks/use-defpool-api";

const WorkersPanel = () => {
  // For demo purposes, using a wallet from environment. In production, this should be configurable
  const demoWallet = import.meta.env.VITE_DEMO_WALLET || "44AFFq5kSiGBoZ4NMDwYtN18obc8AemS33DBDDws8keQf66JxvVXuquhE3mAyUAL4f8cpAGzBVCTLG0P5sqDK17I3wcBiRT";
  const { data: workers, isLoading, error } = useMinerWorkers(demoWallet);

  const getWorkerStatus = (lastSeen?: string): "online" | "offline" | "warning" => {
    if (!lastSeen) return "offline";
    const lastSeenDate = new Date(lastSeen);
    const now = new Date();
    const minutesSinceLastSeen = (now.getTime() - lastSeenDate.getTime()) / (1000 * 60);

    if (minutesSinceLastSeen < 5) return "online";
    if (minutesSinceLastSeen < 15) return "warning";
    return "offline";
  };

  const statusColor = (status: string) => {
    if (status === "online") return "tui-value-up";
    if (status === "warning") return "tui-value-warn";
    return "tui-value-down";
  };

  const onlineCount = workers?.filter(w => getWorkerStatus(w.last_seen) === "online").length || 0;
  const totalCount = workers?.length || 0;

  if (isLoading) {
    return (
      <div className="tui-window h-full">
        <div className="tui-title">[ WORKERS - 0/0 ONLINE ]</div>
        <div className="tui-content">
          <table className="tui-table">
            <thead>
              <tr>
                <th>NAME</th>
                <th className="text-right">MH/s</th>
                <th className="text-right">SHARES</th>
                <th className="text-right">LAST SEEN</th>
                <th className="text-right">STATUS</th>
              </tr>
            </thead>
            <tbody>
              {Array.from({ length: 5 }, (_, i) => (
                <tr key={i}>
                  <td className="text-muted-foreground">Loading...</td>
                  <td className="text-right">0.00</td>
                  <td className="text-right">0</td>
                  <td className="text-right">-</td>
                  <td className="text-right text-muted-foreground">OFFLINE</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="tui-window h-full">
        <div className="tui-title">[ WORKERS ]</div>
        <div className="tui-content flex items-center justify-center">
          <span className="text-red-500">Error loading workers</span>
        </div>
      </div>
    );
  }

  return (
    <div className="tui-window h-full">
      <div className="tui-title">[ WORKERS - {onlineCount}/{totalCount} ONLINE ]</div>
      <div className="tui-content">
        <table className="tui-table">
          <thead>
            <tr>
              <th>NAME</th>
              <th className="text-right">MH/s</th>
              <th className="text-right">SHARES</th>
              <th className="text-right">LAST SEEN</th>
              <th className="text-right">STATUS</th>
            </tr>
          </thead>
          <tbody>
            {workers && workers.length > 0 ? (
              workers.map(w => {
                const status = getWorkerStatus(w.last_seen);
                return (
                  <tr key={w.id}>
                    <td>{w.worker_name}</td>
                    <td className="text-right">{(w.hashrate / 1000000).toFixed(2)}</td>
                    <td className="text-right">{w.total_shares.toLocaleString()}</td>
                    <td className="text-right">
                      {w.last_seen ? new Date(w.last_seen).toLocaleTimeString() : "Never"}
                    </td>
                    <td className={`text-right ${statusColor(status)}`}>
                      {status.toUpperCase()}
                    </td>
                  </tr>
                );
              })
            ) : (
              <tr>
                <td colSpan={5} className="text-center text-muted-foreground py-4">
                  No workers found for wallet: {demoWallet.slice(0, 8)}...
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default WorkersPanel;
