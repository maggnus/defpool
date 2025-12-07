import { useState, useEffect, useCallback } from "react";
import StatusBar from "@/components/tui/StatusBar";
import CurrentMining from "@/components/tui/CurrentMining";
import ProfitabilityTable from "@/components/tui/ProfitabilityTable";
import WorkersPanel from "@/components/tui/WorkersPanel";
import EarningsPanel from "@/components/tui/EarningsPanel";
import PoolStats from "@/components/tui/PoolStats";
import SwitchLog from "@/components/tui/SwitchLog";
import HashrateGraph from "@/components/tui/HashrateGraph";
import CommandBar from "@/components/tui/CommandBar";
import Onboarding from "@/components/tui/Onboarding";
import SettingsMenu from "@/components/tui/menus/SettingsMenu";
import WalletsMenu from "@/components/tui/menus/WalletsMenu";
import PayoutMenu from "@/components/tui/menus/PayoutMenu";
import HelpMenu from "@/components/tui/menus/HelpMenu";

const Index = () => {
  const [showOnboarding, setShowOnboarding] = useState(() => {
    return localStorage.getItem("automine_onboarded") !== "true";
  });
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [walletsOpen, setWalletsOpen] = useState(false);
  const [payoutOpen, setPayoutOpen] = useState(false);
  const [helpOpen, setHelpOpen] = useState(false);
  const [refreshKey, setRefreshKey] = useState(0);

  const handleOnboardingComplete = () => {
    localStorage.setItem("automine_onboarded", "true");
    setShowOnboarding(false);
  };

  const handleOnboardingSkip = () => {
    setShowOnboarding(false);
  };

  const handleRefresh = useCallback(() => {
    setRefreshKey((prev) => prev + 1);
  }, []);

  const isAnyModalOpen = settingsOpen || walletsOpen || payoutOpen || helpOpen || showOnboarding;

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (isAnyModalOpen) return;
      
      switch (e.key.toLowerCase()) {
        case "s":
          setSettingsOpen(true);
          break;
        case "w":
          setWalletsOpen(true);
          break;
        case "p":
          setPayoutOpen(true);
          break;
        case "r":
          handleRefresh();
          break;
        case "h":
          setHelpOpen(true);
          break;
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [isAnyModalOpen, handleRefresh]);

  return (
    <div className="h-screen flex flex-col bg-background overflow-hidden">
      <StatusBar />

      <div
        key={refreshKey}
        className="flex-1 p-1 grid grid-cols-12 grid-rows-6 gap-1 min-h-0"
      >
        {/* Top row */}
        <div className="col-span-3 row-span-2">
          <CurrentMining />
        </div>
        <div className="col-span-5 row-span-2">
          <ProfitabilityTable />
        </div>
        <div className="col-span-4 row-span-2">
          <EarningsPanel />
        </div>

        {/* Middle row */}
        <div className="col-span-6 row-span-2">
          <WorkersPanel />
        </div>
        <div className="col-span-3 row-span-2">
          <PoolStats />
        </div>
        <div className="col-span-3 row-span-2">
          <HashrateGraph />
        </div>

        {/* Bottom row */}
        <div className="col-span-12 row-span-2">
          <SwitchLog />
        </div>
      </div>

      <CommandBar
        onOpenSettings={() => setSettingsOpen(true)}
        onOpenWallets={() => setWalletsOpen(true)}
        onOpenPayout={() => setPayoutOpen(true)}
        onRefresh={handleRefresh}
        onOpenHelp={() => setHelpOpen(true)}
      />

      {showOnboarding && (
        <Onboarding 
          onComplete={handleOnboardingComplete} 
          onSkip={handleOnboardingSkip}
        />
      )}

      <SettingsMenu isOpen={settingsOpen} onClose={() => setSettingsOpen(false)} />
      <WalletsMenu isOpen={walletsOpen} onClose={() => setWalletsOpen(false)} />
      <PayoutMenu isOpen={payoutOpen} onClose={() => setPayoutOpen(false)} />
      <HelpMenu isOpen={helpOpen} onClose={() => setHelpOpen(false)} />
    </div>
  );
};

export default Index;
