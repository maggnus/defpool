import { useEffect, useState } from "react";

interface Worker {
  name: string;
  hashrate: number;
  temp: number;
  power: number;
  status: "online" | "offline" | "warning";
}

const initialWorkers: Worker[] = [
  { name: "rig-01", hashrate: 312.4, temp: 62, power: 245, status: "online" },
  { name: "rig-02", hashrate: 298.7, temp: 65, power: 238, status: "online" },
  { name: "rig-03", hashrate: 156.2, temp: 71, power: 189, status: "warning" },
  { name: "rig-04", hashrate: 0, temp: 0, power: 0, status: "offline" },
  { name: "rig-05", hashrate: 289.1, temp: 58, power: 232, status: "online" },
];

const WorkersPanel = () => {
  const [workers, setWorkers] = useState(initialWorkers);

  useEffect(() => {
    const timer = setInterval(() => {
      setWorkers(prev => prev.map(w => w.status !== "offline" ? ({
        ...w,
        hashrate: Math.max(0, w.hashrate + (Math.random() - 0.5) * 10),
        temp: Math.max(40, Math.min(85, w.temp + (Math.random() - 0.5) * 2)),
      }) : w));
    }, 2000);
    return () => clearInterval(timer);
  }, []);

  const statusColor = (s: string) => {
    if (s === "online") return "tui-value-up";
    if (s === "warning") return "tui-value-warn";
    return "tui-value-down";
  };

  return (
    <div className="tui-window h-full">
      <div className="tui-title">[ WORKERS - 4/5 ONLINE ]</div>
      <div className="tui-content">
        <table className="tui-table">
          <thead>
            <tr>
              <th>NAME</th>
              <th className="text-right">MH/s</th>
              <th className="text-right">TEMP</th>
              <th className="text-right">WATT</th>
              <th className="text-right">STATUS</th>
            </tr>
          </thead>
          <tbody>
            {workers.map(w => (
              <tr key={w.name}>
                <td>{w.name}</td>
                <td className="text-right">{w.hashrate.toFixed(1)}</td>
                <td className={`text-right ${w.temp > 70 ? "tui-value-warn" : ""}`}>
                  {w.temp > 0 ? `${w.temp}°C` : "-"}
                </td>
                <td className="text-right">{w.power > 0 ? `${w.power}W` : "-"}</td>
                <td className={`text-right ${statusColor(w.status)}`}>
                  {w.status.toUpperCase()}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default WorkersPanel;
