import { useEffect, useState } from "react";

const HashrateGraph = () => {
  const data: number[] = Array(48).fill(0); // Show zeros for all time periods

  return (
    <div className="tui-window h-full">
      <div className="tui-title">[ HASHRATE 24H - 0.0% AVG ]</div>
      <div className="tui-content">
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
                    <span key={i} className={val > (5 - row) * 20 ? "text-muted-foreground" : "text-transparent"}>
                      {val > (5 - row) * 20 ? "▁" : " "}
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
      </div>
    </div>
  );
};

export default HashrateGraph;
