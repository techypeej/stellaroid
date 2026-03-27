# Cashtellar
**Verification layer for SEA e-wallet receipts using Soroban.**

## Overview
**Problem:** Scammers edit GCash/e-wallet screenshots or reuse old receipts to trick merchants.
**Solution:** Cashtellar uses Soroban smart contracts to register unique receipt hashes, ensuring every transaction is authentic and single-use.

## MVP Timeline
- **Week 1:** Smart Contract development (Hash registration & duplicate checks).
- **Week 2:** Integration with a simple No-Code frontend (Bubble/FlutterFlow) via Stellar SDK.
- **Week 3:** Beta testing with local MSME vendors.

## Technical Details
- **Stellar Features:** Soroban Smart Contracts (Rust).
- **Prerequisites:** - Rust toolchain
  - Soroban CLI (`cargo install --locked soroban-cli`)

## Instructions
- **Build:** `soroban contract build`
- **Test:** `cargo test`
- **Deploy:** `soroban contract deploy --network testnet --source alice`

## Sample Invocation
```bash
soroban contract invoke \
  --id [CONTRACT_ID] \
  --source user_account \
  --network testnet \
  -- \
  register_receipt \
  --owner [USER_ADDRESS] \
  --receipt_hash "receipt_id_001"