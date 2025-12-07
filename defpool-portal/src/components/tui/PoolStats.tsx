import { useEffect, useState } from "react";

const PoolStats = () => {
  const [hashrate, setHashrate] = useState(1247.8);
  const [miners, setMiners] = useState(12847);

  useEffect(() => {
    const timer = setInterval(() => {
      setHashrate(prev => prev + (Math.random() - 0.5) * 5);
      setMiners(prev => prev + Math.floor((Math.random() - 0.3) * 3));
    }, 3000);
    return () => clearInterval(timer);
  }, []);

  return (
    <div className="tui-window h-full">
      <div className="tui-title">[ POOL STATS ]</div>
      <div className="tui-content space-y-1">
        <div className="tui-row">
          <span className="tui-label">POOL HASH</span>
          <span className="tui-value">{hashrate.toFixed(1)} TH/s</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">MINERS</span>
          <span className="tui-value">{miners.toLocaleString()}</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">BLOCKS/24H</span>
          <span className="tui-value">247</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">LUCK</span>
          <span className="tui-value-up">102.4%</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">FEE</span>
          <span className="tui-value">0.9%</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">MIN PAYOUT</span>
          <span className="tui-value">0.001 BTC</span>
        </div>
      </div>
    </div>
  );
};

export default PoolStats;
