const EarningsPanel = () => {
  return (
    <div className="tui-window h-full">
      <div className="tui-title">[ EARNINGS ]</div>
      <div className="tui-content space-y-1">
        <div className="tui-row">
          <span className="tui-label">TODAY</span>
          <span className="tui-value">$0.00</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">YESTERDAY</span>
          <span className="tui-value">$0.00</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">THIS WEEK</span>
          <span className="tui-value">$0.00</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">THIS MONTH</span>
          <span className="tui-value">$0.00</span>
        </div>
        <div className="border-t border-border my-2" />
        <div className="tui-row">
          <span className="tui-label">UNPAID</span>
          <span className="tui-value-up">0.00000000 BTC</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">TOTAL PAID</span>
          <span className="tui-value">0.00000000 BTC</span>
        </div>
      </div>
    </div>
  );
};

export default EarningsPanel;
