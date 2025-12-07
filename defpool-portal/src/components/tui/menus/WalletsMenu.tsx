import { useState } from "react";
import TuiModal from "../TuiModal";
import TuiInput from "../TuiInput";
import TuiButton from "../TuiButton";

interface Wallet {
  coin: string;
  address: string;
}

interface WalletsMenuProps {
  isOpen: boolean;
  onClose: () => void;
}

const WalletsMenu = ({ isOpen, onClose }: WalletsMenuProps) => {
  const [wallets, setWallets] = useState<Wallet[]>([]);
  const [newCoin, setNewCoin] = useState("");
  const [newAddress, setNewAddress] = useState("");

  const addWallet = () => {
    if (newCoin && newAddress) {
      setWallets([...wallets, { coin: newCoin.toUpperCase(), address: newAddress }]);
      setNewCoin("");
      setNewAddress("");
    }
  };

  const removeWallet = (index: number) => {
    setWallets(wallets.filter((_, i) => i !== index));
  };

  return (
    <TuiModal title="WALLETS" isOpen={isOpen} onClose={onClose} width="max-w-2xl">
      <div className="space-y-3">
        <table className="tui-table">
          <thead>
            <tr>
              <th className="w-20">COIN</th>
              <th>ADDRESS</th>
              <th className="w-16">ACTION</th>
            </tr>
          </thead>
          <tbody>
            {wallets.map((wallet, index) => (
              <tr key={index}>
                <td className="text-foreground">{wallet.coin}</td>
                <td className="text-muted-foreground font-mono text-xs">
                  {wallet.address.slice(0, 30)}...
                </td>
                <td>
                  <button
                    onClick={() => removeWallet(index)}
                    className="text-destructive hover:underline"
                  >
                    [DEL]
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>

        <div className="border-t border-border pt-4 mt-4">
          <div className="text-muted-foreground text-xs mb-2">ADD NEW WALLET</div>
          <div className="flex gap-2">
            <input
              value={newCoin}
              onChange={(e) => setNewCoin(e.target.value)}
              placeholder="COIN"
              className="w-20 bg-secondary border border-border px-2 py-1 text-foreground focus:outline-none focus:border-foreground"
            />
            <input
              value={newAddress}
              onChange={(e) => setNewAddress(e.target.value)}
              placeholder="Wallet address"
              className="flex-1 bg-secondary border border-border px-2 py-1 text-foreground focus:outline-none focus:border-foreground"
            />
            <TuiButton onClick={addWallet}>Add</TuiButton>
          </div>
        </div>

        <div className="flex gap-2 pt-4 justify-end">
          <TuiButton variant="primary" onClick={onClose}>
            Done
          </TuiButton>
        </div>
      </div>
    </TuiModal>
  );
};

export default WalletsMenu;
