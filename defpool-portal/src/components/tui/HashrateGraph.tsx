import { useEffect, useState } from "react";

const HashrateGraph = () => {
  const [data, setData] = useState<number[]>(Array(48).fill(0).map(() => 70 + Math.random() * 30));

  useEffect(() => {
    const timer = setInterval(() => {
      setData(prev => [...prev.slice(1), 70 + Math.random() * 30]);
    }, 2000);
    return () => clearInterval(timer);
  }, []);

  const max = Math.max(...data);
  const min = Math.min(...data);

  const getChar = (value: number, row: number) => {
    const normalized = (value - min) / (max - min);
    const height = Math.floor(normalized * 6);
    if (height >= (5 - row)) return "█";
    return " ";
  };

  return (
    <div className="tui-window h-full">
      <div className="tui-title">[ HASHRATE 24H - {data[data.length - 1].toFixed(0)}% AVG ]</div>
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
                    <span key={i} className={getChar(val, row) === "█" ? "text-foreground" : "text-transparent"}>
                      {getChar(val, row)}
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
