# Credit Token Program

This is the source code for the Credit Token Program deployed on Solana mainnet.

## Deployed Program IDs

| Network    | Program ID                                      | Explorer Link                                                                                  |
|------------|------------------------------------------------|-----------------------------------------------------------------------------------------------|
| Mainnet    | `FPgBsgLFt5PTDHKnC38YWgoPJy4pJ111SgDELPKrPGpw` | [Solana Explorer](https://explorer.solana.com/address/FPgBsgLFt5PTDHKnC38YWgoPJy4pJ111SgDELPKrPGpw) |
| Mainnet (older version) | `HJWf9ne42BeCtCWti7VmydDs9jgnZxiTtMUBJTxb29g5` | [Solana Explorer](https://explorer.solana.com/address/HJWf9ne42BeCtCWti7VmydDs9jgnZxiTtMUBJTxb29g5) |

## Program Details
The Credit Token Program allows users to purchase credits using SOL. The program includes functionality for:

1. Initializing a vault
2. Purchasing credits in various amounts
3. Allowing the authority to withdraw SOL from the vault

## Building and Verifying

To build the program:
```bash
anchor build
```

Due to ARM64/AMD64 architecture differences, verification using `solana-verify` directly on Apple Silicon Macs may encounter Docker compatibility issues.

## Security

For security concerns, please contact maintainers via the details provided in the program's security.txt.

## Overview
The Credit Token Program is a Solana smart contract designed for secure and efficient credit token purchases. It enables users to purchase credits with SOL and manages withdrawals through a secure vault system, ensuring reliable and transparent transactions on the Solana blockchain.

## Features
- Purchase credits with SOL at predefined rates
- Secure vault management through PDAs (Program Derived Addresses)
- Authority-controlled withdrawals with multi-signature support
- Comprehensive input validation and security measures
- Real-time transaction processing and confirmation

## Architecture

### Accounts
- Vault: Stores SOL from credit purchases
- Authority: Controls withdrawal permissions
- User: Purchases credits with SOL

### Instructions
1. Initialize Vault
   - Creates secure vault for storing SOL
   - Sets up authority permissions
2. Purchase Credits
   - Validates SOL amount
   - Calculates credit allocation
   - Transfers SOL to vault
3. Withdraw Funds
   - Requires authority signature
   - Validates withdrawal amount
   - Transfers SOL from vault

## Credit Purchase Rates
- 1 SOL = 100 credits
- 5 SOL = 550 credits (10onus)
- 10 SOL = 1200 credits (20onus)

## Build and Deploy

### Prerequisites
- Rust 1.70.0 or later
- Solana CLI 1.17.0 or later
- Anchor Framework 0.29.0 or later

### Build Commands
```bash
anchor build
anchor test
```

### Verify Build
To ensure the build matches the deployed program:
```bash
anchor verify <PROGRAM_ID>
```

### Test Commands
```bash
anchor test
```