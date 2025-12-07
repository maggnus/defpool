import { useEffect, useState } from "react";

const StatusBar = () => {
  const [time, setTime] = useState(new Date());

  useEffect(() => {
    const timer = setInterval(() => setTime(new Date()), 1000);
    return () => clearInterval(timer);
  }, []);

  return (
    <div className="flex justify-between items-center px-2 py-1 border-b border-border bg-secondary text-xs">
      <div className="flex gap-4">
        <span className="text-muted-foreground">AUTOMINE v2.4.1</span>
        <span className="text-success">● CONNECTED</span>
      </div>
      <div className="flex gap-4">
        <span className="text-muted-foreground">POOL: us-east.automine.io:3333</span>
        <span>{time.toLocaleTimeString()}</span>
      </div>
    </div>
  );
};

export default StatusBar;
