const PoolStats = () => {
  return (
    <div className="tui-window h-full">
      <div className="tui-title">[ POOL STATS ]</div>
      <div className="tui-content space-y-1">
        <div className="flex items-center justify-center h-full">
          <div className="text-center space-y-2">
            <div className="text-muted-foreground text-sm">[NO ACTIVE POOL]</div>
            <div className="text-muted-foreground text-xs">
              Pool statistics will be displayed when<br/>
              a mining target is active and connected
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default PoolStats;
