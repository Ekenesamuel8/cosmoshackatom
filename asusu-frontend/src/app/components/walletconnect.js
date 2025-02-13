import { useState } from "react";

const WalletConnect = () => {
  const [walletAddress, setWalletAddress] = useState(null);

  const connectWallet = async () => {
    if (!window.keplr) {
      alert("Please install Keplr Wallet extension.");
      return;
    }

    try {
      // Suggest Cosmos chain if not added
      await window.keplr.enable("cosmoshub-4");
      const offlineSigner = window.getOfflineSigner("cosmoshub-4");
      const accounts = await offlineSigner.getAccounts();

      setWalletAddress(accounts[0].address);
    } catch (error) {
      console.error("Failed to connect wallet:", error);
    }
  };

  return (
    <div className="flex flex-col items-center p-4">
      {walletAddress ? (
        <p className="text-green-500">Connected: {walletAddress}</p>
      ) : (
        <button
          onClick={connectWallet}
          className="px-4 py-2 bg-blue-600 text-white rounded-lg"
        >
          Connect Wallet
        </button>
      )}
    </div>
  );
};

export default WalletConnect;
