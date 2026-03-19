# Crowdfunding_dApp
# 🚀 Crowdfunding dApp on Stellar (Soroban)

## 📌 Project Description

This project is a decentralized crowdfunding application built using **Soroban smart contracts on the Stellar blockchain**. It allows users to create and fund campaigns in a trustless environment where all transactions and contributions are recorded on-chain.

The goal of this project is to demonstrate how decentralized finance (DeFi) concepts can be implemented using Soroban smart contracts.

---

## ⚙️ What It Does

The smart contract enables a basic crowdfunding workflow:

* A campaign creator initializes a fundraising campaign with a funding goal and duration
* Users can contribute funds to the campaign
* Contributions are tracked individually on-chain
* If the funding goal is reached, the creator can withdraw funds
* If the goal is not met before the deadline, contributors can claim refunds

---

## ✨ Features

* 🔐 **Decentralized & Transparent**

  * No intermediaries; all logic runs on-chain

* 👤 **Campaign Initialization**

  * Creator sets funding goal and duration

* 💸 **User Contributions**

  * Multiple users can fund the campaign

* 📊 **On-Chain Tracking**

  * Individual contributions stored securely

* 🎯 **Goal-Based Withdrawal**

  * Funds released only if the goal is achieved

* 🔁 **Refund Mechanism**

  * Contributors can withdraw funds if campaign fails

---

## 🛠️ Tech Stack

* **Blockchain:** Stellar
* **Smart Contracts:** Soroban
* **Language:** Rust
* **SDK:** soroban-sdk

---

## 🚀 Getting Started

### Prerequisites

* Rust installed
* Soroban CLI installed
* Stellar testnet account

### Build Contract

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Deploy Contract

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/your_contract.wasm \
  --source your-account
```

---

## 🔗 Deployed Smart Contract

**Contract Address:**
https://stellar.expert/explorer/testnet/contract/CAO3SAXF263VPZOBEQJ22LWBJCPXJUHWRRYP3VELTOUSI7I5KXG7BZBI
<img width="1890" height="932" alt="Screenshot 2026-03-19 144622" src="https://github.com/user-attachments/assets/a990fb0a-57bc-4f23-8c81-142302edea8b" />


*(Replace this with your deployed contract ID)*

---

## 📈 Future Improvements

* Support for multiple campaigns
* Real token transfers (XLM / custom tokens)
* Frontend integration (React + Freighter wallet)
* Campaign metadata (title, description, images)
* Milestone-based fund release
* Voting/governance by contributors

---

## 🤝 Contributing

Contributions are welcome! Feel free to fork the repository and submit pull requests.

---

## 📜 License

MIT License
