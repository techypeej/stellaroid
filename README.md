Cashtellar: Decentralized Receipt Verification
Cashtellar is a Soroban-based smart contract deployed on the Stellar Network. It serves as a decentralized "Truth Engine" to combat the rising trend of fake e-wallet receipts and transaction scams. By anchoring unique transaction fingerprints (hashes) onto a public ledger, Cashtellar ensures that digital receipts are immutable and verifiable by any merchant or user.

🛡️ The Problem: Receipt Fraud
In many digital payment ecosystems, scammers use image editing software to modify screenshots of successful transactions. They alter the amount, date, or sender information to deceive sellers into releasing goods or services without actual payment.

💡 The Solution
Cashtellar provides a Receipt Anchor & Verify protocol:

Anchor: When a transaction occurs, the system generates a unique hash and registers it on the Stellar blockchain.

Verify: A merchant can query the smart contract using the Receipt ID. Since the blockchain is immutable, the data returned is the absolute truth, regardless of what a screenshot shows.

🚀 Deployment Information
The contract has been successfully compiled and deployed to the Stellar Testnet.

Network: Stellar Testnet

Contract ID: CAG7XYQQ5WP7H2ZXRFX733FP5BETSDO5V3CG7V4DASQO326E6WLXDVKS

Deployment Status: Verified

Transaction URL: [View on Stellar Expert](https://stellar.expert/explorer/testnet/tx/c6ea215f84d16caaa29bd1bdfeea59112dcad61b1c3b2ad32424f8265a0be058)

🛠️ Technical Setup
Prerequisites
Rust Toolchain: v1.94.1 

Stellar CLI: Installed and configured for testnet

Target: wasm32v1-none

Installation & Build
To replicate the environment and build the contract locally:

Bash
# 1. Clone the repository
git clone https://github.com/your-username/stellaroid.git

# 2. Build the contract
stellar contract build

# 3. Optimize the Wasm file
stellar contract optimize --wasm target/wasm32v1-none/release/stellaroid.wasm

Dependency Management
This project uses specific version pinning in Cargo.toml to prevent "Edition 2024" compatibility issues with the current Soroban SDK.

🖥️ Usage
Registering a Receipt
To anchor a new transaction to the blockchain:

Bash
stellar contract invoke \
  --id CAG7XYQQ5WP7H2ZXRFX733FP5BETSDO5V3CG7V4DASQO326E6WLXDVKS \
  --source <YOUR_KEY> \
  --network testnet \
  -- \
  register_receipt --owner <USER_ADDRESS> --amount <AMOUNT>
  
🎓 About the Developer
Developed by Paul John Sianghio, a 3rd-year Computer Engineering student at the University of the East - Caloocan. This project was built to explore the intersection of Cybersecurity and Blockchain technology in the Philippine financial landscape.