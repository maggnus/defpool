import { useEffect, useState } from "react";

const CurrentMining = () => {
  const [hashrate, setHashrate] = useState(847.32);
  const [accepted, setAccepted] = useState(12847);
  const [rejected, setRejected] = useState(23);

  useEffect(() => {
    const timer = setInterval(() => {
      setHashrate(prev => prev + (Math.random() - 0.5) * 5);
      if (Math.random() > 0.7) setAccepted(prev => prev + 1);
      if (Math.random() > 0.98) setRejected(prev => prev + 1);
    }, 2000);
    return () => clearInterval(timer);
  }, []);

  return (
    <div className="tui-window h-full">
      <div className="tui-title">[ CURRENT MINING ]</div>
      <div className="tui-content space-y-1">
        <div className="tui-row">
          <span className="tui-label">COIN</span>
          <span className="tui-value">KASPA (KAS)</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">ALGO</span>
          <span className="tui-value">kHeavyHash</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">HASHRATE</span>
          <span className="tui-value">{hashrate.toFixed(2)} MH/s</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">ACCEPTED</span>
          <span className="tui-value-up">{accepted.toLocaleString()}</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">REJECTED</span>
          <span className="tui-value-down">{rejected}</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">UPTIME</span>
          <span className="tui-value">3d 14h 27m</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">EFFICIENCY</span>
          <span className="tui-value-up">99.82%</span>
        </div>
      </div>
    </div>
  );
};

export default CurrentMining;
