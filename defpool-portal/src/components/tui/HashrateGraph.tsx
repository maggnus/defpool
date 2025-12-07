import { useEffect, useState } from "react";

const HashrateGraph = () => {
  const data: number[] = []; // No data available yet

  return (
    <div className="tui-window h-full">
      <div className="tui-title">[ HASHRATE 24H ]</div>
      <div className="tui-content">
        {data.length > 0 ? (
          <div className="flex gap-0.5 text-muted-foreground leading-none">
            <div className="flex flex-col text-right pr-1 text-[10px]">
              <span>100</span>
              <span className="flex-1" />
              <span>50</span>
            </div>
            <div className="flex-1">
              <div className="flex flex-col">
                {[0, 1, 2, 3, 4, 5].map(row => (
                  <div key={row} className="flex text-foreground leading-none" style={{ fontSize: "8px" }}>
                    {data.map((val, i) => (
                      <span key={i} className="text-transparent">
                        {" "}
                      </span>
                    ))}
                  </div>
                ))}
              </div>
              <div className="border-t border-border mt-1 pt-1 flex justify-between text-[10px]">
                <span>-24h</span>
                <span>-12h</span>
                <span>NOW</span>
              </div>
            </div>
          </div>
        ) : (
          <div className="flex items-center justify-center h-full">
            <div className="text-center space-y-2">
              <div className="text-muted-foreground text-sm">[NO DATA]</div>
              <div className="text-muted-foreground text-xs">
                Hashrate graph will appear when<br/>
                mining activity is detected
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default HashrateGraph;
