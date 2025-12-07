import { useEffect, useState } from "react";

interface CoinData {
  coin: string;
  algo: string;
  profit: number;
  change: number;
  active: boolean;
}

const initialCoins: CoinData[] = [
  { coin: "KAS", algo: "kHeavyHash", profit: 2.47, change: 5.2, active: true },
  { coin: "RVN", algo: "KawPow", profit: 2.31, change: -1.4, active: false },
  { coin: "ERG", algo: "Autolykos", profit: 2.18, change: 3.1, active: false },
  { coin: "FLUX", algo: "ZelHash", profit: 1.94, change: 0.8, active: false },
  { coin: "ETC", algo: "Etchash", profit: 1.87, change: -0.3, active: false },
  { coin: "NEXA", algo: "NexaPow", profit: 1.76, change: 2.4, active: false },
  { coin: "CLORE", algo: "KawPow", profit: 1.65, change: -2.1, active: false },
  { coin: "NEOX", algo: "KawPow", profit: 1.52, change: 1.2, active: false },
];

const ProfitabilityTable = () => {
  const [coins, setCoins] = useState(initialCoins);

  useEffect(() => {
    const timer = setInterval(() => {
      setCoins(prev => prev.map(coin => ({
        ...coin,
        profit: Math.max(0.5, coin.profit + (Math.random() - 0.5) * 0.1),
        change: coin.change + (Math.random() - 0.5) * 0.5,
      })).sort((a, b) => b.profit - a.profit).map((c, i) => ({
        ...c,
        active: i === 0
      })));
    }, 3000);
    return () => clearInterval(timer);
  }, []);

  return (
    <div className="tui-window h-full">
      <div className="tui-title">[ PROFITABILITY - $/DAY/100MH ]</div>
      <div className="tui-content">
        <table className="tui-table">
          <thead>
            <tr>
              <th>#</th>
              <th>COIN</th>
              <th>ALGO</th>
              <th className="text-right">$/DAY</th>
              <th className="text-right">24H</th>
            </tr>
          </thead>
          <tbody>
            {coins.map((coin, i) => (
              <tr key={coin.coin} className={coin.active ? "bg-secondary" : ""}>
                <td className="text-muted-foreground">{i + 1}</td>
                <td className={coin.active ? "text-foreground" : "text-muted-foreground"}>
                  {coin.active && "► "}{coin.coin}
                </td>
                <td className="text-muted-foreground">{coin.algo}</td>
                <td className="text-right">${coin.profit.toFixed(2)}</td>
                <td className={`text-right ${coin.change >= 0 ? "tui-value-up" : "tui-value-down"}`}>
                  {coin.change >= 0 ? "+" : ""}{coin.change.toFixed(1)}%
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default ProfitabilityTable;
