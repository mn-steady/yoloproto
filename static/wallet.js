export async function connectKeplrWallet() {
    if (window.keplr) {
        try {
            // Enable the Secret Network in Keplr
            await window.keplr.enable("secret-4");

            // Get the offline signer for the Secret Network
            const offlineSigner = window.getOfflineSigner("secret-4");
            const accounts = await offlineSigner.getAccounts();

            // Log the connected account
            console.log("Connected account:", accounts[0]);
        } catch (error) {
            console.error("Failed to connect to Keplr wallet:", error);
        }
    } else {
        alert("Keplr wallet not found!");
    }
}
