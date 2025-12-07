const PoolStats = () => {
  return (
    <div className="tui-window h-full">
      <div className="tui-title">[ POOL STATS ]</div>
      <div className="tui-content space-y-1">
        <div className="tui-row">
          <span className="tui-label">POOL HASH</span>
          <span className="tui-value">0.0 H/s</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">MINERS</span>
          <span className="tui-value">0</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">BLOCKS/24H</span>
          <span className="tui-value">0</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">LUCK</span>
          <span className="tui-value">0.0%</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">FEE</span>
          <span className="tui-value">0.0%</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">MIN PAYOUT</span>
          <span className="tui-value">0.00000000 BTC</span>
        </div>
      </div>
    </div>
  );
};

export default PoolStats;
