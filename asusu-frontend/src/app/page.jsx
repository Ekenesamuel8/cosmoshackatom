"use client";
import { useState } from "react";
import { connectWallet } from "./components/walletconnect";

export default function Home() {
    const [account, setAccount] = useState(null);

    const handleConnect = async () => {
        const address = await connectWallet();
        if (address) setAccount(address);
    };

    return (
        <section className="flex flex-col items-center justify-center h-[80vh]">
            <h1 className="text-3xl font-bold">Welcome to Asusu Contribution WebApp</h1>
            <p className="mt-2 text-lg text-gray-600">Your trusted savings and contribution platform.</p>

            {account ? (
                <p className="mt-4 text-green-600">Connected: {account}</p>
            ) : (
                <button onClick={handleConnect} className="mt-4 px-6 py-2 bg-blue-600 text-white rounded">
                    Connect Wallet
                </button>
            )}
        </section>
    );
}
