import { useState, useEffect } from "react";
import { useTargets, useCurrentTargetName, ProfitabilityScore } from "@/hooks/use-defpool-api";

interface CoinData {
  coin: string;
  algo: string;
  profit: number;
  change: number;
  active: boolean;
  target_name: string;
}

// Algorithm mapping for display
const getAlgorithmDisplay = (coin: string): string => {
  switch (coin) {
    case "XMR":
      return "RandomX";
    case "LTC":
    case "DOGE":
      return "Scrypt";
    default:
      return "Unknown";
  }
};

const ProfitabilityTable = () => {
  const { data: targets, isLoading, error } = useTargets();
  const { data: currentTarget } = useCurrentTargetName();
  const [previousScores, setPreviousScores] = useState<Map<string, number>>(new Map());

  // Convert API data to component format
  const coins: CoinData[] = targets?.map((target: ProfitabilityScore) => {
    const previousScore = previousScores.get(target.target_name) || target.score;
    const change = previousScore > 0 ? ((target.score - previousScore) / previousScore) * 100 : 0;

    return {
      coin: target.coin,
      algo: getAlgorithmDisplay(target.coin),
      profit: target.score,
      change,
      active: target.target_name === currentTarget,
      target_name: target.target_name,
    };
  }).sort((a, b) => b.profit - a.profit) || [];

  // Update previous scores for change calculation
  useEffect(() => {
    if (targets) {
      const newPreviousScores = new Map();
      targets.forEach((target: ProfitabilityScore) => {
        newPreviousScores.set(target.target_name, target.score);
      });
      setPreviousScores(newPreviousScores);
    }
  }, [targets]);

  if (isLoading) {
    return (
      <div className="tui-window h-full">
        <div className="tui-title">[ PROFITABILITY ]</div>
        <div className="tui-content flex items-center justify-center">
          <span className="text-muted-foreground">Loading...</span>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="tui-window h-full">
        <div className="tui-title">[ PROFITABILITY ]</div>
        <div className="tui-content flex items-center justify-center">
          <span className="text-red-500">Error loading data</span>
        </div>
      </div>
    );
  }

  return (
    <div className="tui-window h-full">
      <div className="tui-title">[ PROFITABILITY SCORES ]</div>
      <div className="tui-content">
        <table className="tui-table">
          <thead>
            <tr>
              <th>#</th>
              <th>POOL</th>
              <th>COIN</th>
              <th>ALGO</th>
              <th className="text-right">SCORE</th>
              <th className="text-right">CHANGE</th>
            </tr>
          </thead>
          <tbody>
            {coins.map((coin, i) => (
              <tr key={coin.target_name} className={coin.active ? "bg-secondary" : ""}>
                <td className="text-muted-foreground">{i + 1}</td>
                <td className={coin.active ? "text-foreground" : "text-muted-foreground"}>
                  {coin.active && "► "}{coin.target_name}
                </td>
                <td className="text-muted-foreground">{coin.coin}</td>
                <td className="text-muted-foreground">{coin.algo}</td>
                <td className="text-right">{coin.profit.toFixed(6)}</td>
                <td className={`text-right ${coin.change >= 0 ? "tui-value-up" : "tui-value-down"}`}>
                  {coin.change >= 0 ? "+" : ""}{coin.change.toFixed(2)}%
                </td>
              </tr>
            ))}
            {coins.length === 0 && (
              <tr>
                <td colSpan={6} className="text-center text-muted-foreground py-4">
                  No mining targets available
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default ProfitabilityTable;
