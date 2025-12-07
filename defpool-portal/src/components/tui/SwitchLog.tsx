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
            {logs.length > 0 ? (
              logs.map((log, i) => (
                <tr key={i}>
                  <td className="text-muted-foreground">{log.time}</td>
                  <td>{log.from}</td>
                  <td className="tui-value-up">{log.to}</td>
                  <td className="text-muted-foreground">{log.reason}</td>
                </tr>
              ))
            ) : (
              <tr>
                <td colSpan={4} className="text-center text-muted-foreground py-8">
                  No switching activity yet
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default SwitchLog;
