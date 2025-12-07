import { useState } from "react";
import TuiButton from "./TuiButton";
import TuiInput from "./TuiInput";

interface OnboardingProps {
  onComplete: () => void;
  onSkip: () => void;
}

const Onboarding = ({ onComplete, onSkip }: OnboardingProps) => {
  const [step, setStep] = useState(0);
  const [username, setUsername] = useState("");
  const [btcAddress, setBtcAddress] = useState("");

  const steps = [
    {
      title: "WELCOME",
      content: (
        <div className="space-y-4">
          <div className="text-center py-4">
            <pre className="text-foreground text-xs leading-tight">
{`
    _   _   _ _____ ___  __  __ ___ _   _ ___ 
   / \\ | | | |_   _/ _ \\|  \\/  |_ _| \\ | | __|
  / _ \\| |_| | | || | | | |\\/| || ||  \\| | _| 
 / ___ \\  _  | | || |_| | |  | || || |\\  | |__
/_/   \\_\\_| |_| |_| \\___/|_|  |_|___|_| \\_|____|
`}
            </pre>
          </div>
          <div className="text-center text-muted-foreground">
            <p>Auto-switching cryptocurrency mining pool</p>
            <p className="mt-2">Create an account to track your miners</p>
          </div>
        </div>
      ),
    },
    {
      title: "CREATE ACCOUNT",
      content: (
        <div className="space-y-4">
          <div className="text-muted-foreground text-xs mb-4">
            Enter your username to create an account
          </div>
          <TuiInput
            label="USERNAME"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            placeholder="Enter username"
          />
        </div>
      ),
    },
    {
      title: "PRIMARY WALLET",
      content: (
        <div className="space-y-4">
          <div className="text-muted-foreground text-xs mb-4">
            Enter your primary Bitcoin wallet address for payouts
          </div>
          <TuiInput
            label="BTC ADDRESS"
            value={btcAddress}
            onChange={(e) => setBtcAddress(e.target.value)}
            placeholder="bc1q..."
          />
          <div className="text-muted-foreground text-xs">
            <p>You can add more wallets later in the Wallets menu [W]</p>
          </div>
        </div>
      ),
    },
    {
      title: "SETUP COMPLETE",
      content: (
        <div className="space-y-4 text-center">
          <div className="text-foreground text-2xl py-4">[OK]</div>
          <div className="text-foreground">Your account is ready!</div>
          <div className="text-muted-foreground text-xs space-y-2">
            <p>Username: {username || "Not set"}</p>
            <p>Wallet: {btcAddress ? `${btcAddress.slice(0, 10)}...` : "Not set"}</p>
          </div>
          <div className="text-muted-foreground text-xs pt-4">
            <p>Configure your mining pool in Settings</p>
            <p className="text-foreground mt-2">
              Mining proxy will be available at startup
            </p>
          </div>
        </div>
      ),
    },
  ];

  const currentStep = steps[step];
  const isLastStep = step === steps.length - 1;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-background/90">
      <div className="tui-window w-full max-w-lg mx-4">
        <div className="tui-title flex justify-between">
          <span>[ {currentStep.title} ]</span>
          <span className="text-muted-foreground">
            {step + 1}/{steps.length}
          </span>
        </div>
        <div className="tui-content min-h-[280px] flex flex-col">
          <div className="flex-1">{currentStep.content}</div>
          <div className="flex justify-between pt-4 border-t border-border mt-4">
            <div className="flex gap-2">
              {step > 0 ? (
                <TuiButton onClick={() => setStep(step - 1)}>Back</TuiButton>
              ) : (
                <TuiButton onClick={onSkip}>Skip</TuiButton>
              )}
            </div>
            <TuiButton
              variant="primary"
              onClick={() => {
                if (isLastStep) {
                  onComplete();
                } else {
                  setStep(step + 1);
                }
              }}
            >
              {isLastStep ? "Start Mining" : "Continue"}
            </TuiButton>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Onboarding;
