const EarningsPanel = () => {
  // TODO: Connect to real earnings API when implemented on server
  // For now, showing placeholder data
  return (
    <div className="tui-window h-full">
      <div className="tui-title">[ EARNINGS ]</div>
      <div className="tui-content space-y-1">
        <div className="tui-row">
          <span className="tui-label">TODAY</span>
          <span className="tui-value">$24.87</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">YESTERDAY</span>
          <span className="tui-value">$23.14</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">THIS WEEK</span>
          <span className="tui-value">$156.42</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">THIS MONTH</span>
          <span className="tui-value">$687.91</span>
        </div>
        <div className="border-t border-border my-2" />
        <div className="tui-row">
          <span className="tui-label">UNPAID</span>
          <span className="tui-value-up">0.0847 BTC</span>
        </div>
        <div className="tui-row">
          <span className="tui-label">TOTAL PAID</span>
          <span className="tui-value">2.4721 BTC</span>
        </div>
        <div className="border-t border-border my-2" />
        <div className="text-center text-muted-foreground text-xs py-1">
          Real earnings data coming soon
        </div>
      </div>
    </div>
  );
};

export default EarningsPanel;
