import { SigningStargateClient } from "@cosmjs/stargate";
import { getKeplr } from "./keplr"; // Ensure this function gets Keplr properly

export async function connectWallet() {
  if (!window.keplr) {
      alert("Please install the Keplr wallet extension.");
      return null;
  }

  try {
      // Enable Keplr for the Cosmos chain (replace with actual chain ID)
      const chainId = "cosmoshub-4"; // Change to your chain's ID
      await window.keplr.enable(chainId);

      const offlineSigner = window.keplr.getOfflineSigner(chainId);
      const accounts = await offlineSigner.getAccounts();

      return accounts[0].address; // Return wallet address
  } catch (error) {
      console.error("Wallet connection failed:", error);
      return null;
  }
}


const RPC_ENDPOINT = "https://rpc.cosmos.network"; // Replace with your Cosmos chain RPC
const CONTRACT_ADDRESS = "cosmos1xyz..."; // Replace with your deployed contract address

export const joinGroup = async (groupId) => {
  try {
    // Check if Keplr is available
    const keplr = await getKeplr();
    if (!keplr) {
      throw new Error("Keplr wallet not found. Install it first.");
    }

    // Get user account & signer
    const chainId = "cosmoshub-4"; // Replace with your chain ID
    await keplr.enable(chainId);
    const offlineSigner = keplr.getOfflineSigner(chainId);
    const accounts = await offlineSigner.getAccounts();
    const senderAddress = accounts[0].address;

    // Initialize Stargate Client
    const client = await SigningStargateClient.connectWithSigner(
      RPC_ENDPOINT,
      offlineSigner
    );

    // Prepare transaction message
    const msg = {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: {
        sender: senderAddress,
        contract: CONTRACT_ADDRESS,
        msg: Buffer.from(
          JSON.stringify({ join_group: { group_id: groupId } })
        ).toString("base64"),
        funds: [],
      },
    };

    // Broadcast transaction
    const fee = {
      amount: [{ denom: "uatom", amount: "5000" }], // Adjust for your chain
      gas: "200000",
    };

    const result = await client.signAndBroadcast(senderAddress, [msg], fee);

    console.log("Transaction result:", result);
    return result;
  } catch (error) {
    console.error("Error joining group:", error);
    throw error;
  }
};