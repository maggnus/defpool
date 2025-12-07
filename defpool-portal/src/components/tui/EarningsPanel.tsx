const EarningsPanel = () => {
  return (
    <div className="tui-window h-full">
      <div className="tui-title">[ EARNINGS ]</div>
      <div className="tui-content space-y-1">
        <div className="flex items-center justify-center h-full">
          <div className="text-center space-y-2">
            <div className="text-muted-foreground text-sm">[NO DATA]</div>
            <div className="text-muted-foreground text-xs">
              Earnings tracking will be available when<br/>
              mining activity is detected
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default EarningsPanel;
