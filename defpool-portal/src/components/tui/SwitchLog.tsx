import { useEffect, useState } from "react";

interface LogEntry {
  time: string;
  from: string;
  to: string;
  reason: string;
}

const SwitchLog = () => {
  const [logs, setLogs] = useState<LogEntry[]>([]);

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
            {/* Fill empty rows with zeros */}
            {Array.from({ length: Math.max(0, 6 - logs.length) }, (_, i) => (
              <tr key={`empty-${i}`}>
                <td className="text-muted-foreground">--:--:--</td>
                <td>-</td>
                <td>-</td>
                <td className="text-muted-foreground">-</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default SwitchLog;
