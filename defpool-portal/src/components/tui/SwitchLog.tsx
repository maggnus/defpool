import { useEffect, useState } from "react";

interface LogEntry {
  time: string;
  from: string;
  to: string;
  reason: string;
}

const initialLogs: LogEntry[] = [
  { time: "14:32:17", from: "RVN", to: "KAS", reason: "+8.2% profit" },
  { time: "11:45:02", from: "ERG", to: "RVN", reason: "+3.1% profit" },
  { time: "08:21:44", from: "KAS", to: "ERG", reason: "+5.4% profit" },
  { time: "03:17:28", from: "FLUX", to: "KAS", reason: "+2.7% profit" },
];

const SwitchLog = () => {
  const [logs, setLogs] = useState(initialLogs);

  useEffect(() => {
    const timer = setInterval(() => {
      if (Math.random() > 0.8) {
        const coins = ["KAS", "RVN", "ERG", "FLUX", "ETC", "NEXA"];
        const from = coins[Math.floor(Math.random() * coins.length)];
        let to = coins[Math.floor(Math.random() * coins.length)];
        while (to === from) to = coins[Math.floor(Math.random() * coins.length)];
        
        const now = new Date();
        const newLog = {
          time: now.toLocaleTimeString("en-US", { hour12: false }),
          from,
          to,
          reason: `+${(Math.random() * 10 + 1).toFixed(1)}% profit`
        };
        setLogs(prev => [newLog, ...prev].slice(0, 6));
      }
    }, 5000);
    return () => clearInterval(timer);
  }, []);

  return (
    <div className="tui-window h-full">
      <div className="tui-title">[ SWITCH LOG ]</div>
      <div className="tui-content">
        <table className="tui-table">
          <thead>
            <tr>
              <th>TIME</th>
              <th>FROM</th>
              <th>TO</th>
              <th>REASON</th>
            </tr>
          </thead>
          <tbody>
            {logs.map((log, i) => (
              <tr key={i}>
                <td className="text-muted-foreground">{log.time}</td>
                <td>{log.from}</td>
                <td className="tui-value-up">{log.to}</td>
                <td className="text-muted-foreground">{log.reason}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default SwitchLog;
