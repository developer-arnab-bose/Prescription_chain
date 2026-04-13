# 💊 Prescription Chain (Stellar Soroban)

[![Smart Contract](https://img.shields.io/badge/Soroban-Smart_Contract-blue)](https://soroban.stellar.org/)
[![Network](https://img.shields.io/badge/Network-Stellar_Testnet-lightgrey)](https://stellar.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 📖 Project Description 
**Prescription Chain** is a decentralized healthcare application (dApp) built on the Stellar network using Soroban smart contracts. It tackles the critical industry challenges of prescription fraud, double-filling, and fragmented medical histories by bringing pharmaceutical records securely on-chain. 

By leveraging Stellar's high-speed, low-cost ecosystem, Prescription Chain ensures that medical prescriptions are verifiable, transparent, and securely managed among healthcare providers, patients, and pharmacies—all without relying on a centralized database.

## ⚙️ How It Works 
The Prescription Chain acts as an immutable, transparent ledger for medical prescriptions. The workflow operates in three distinct phases:

1. **Issuance (Doctor):** A licensed healthcare provider creates and cryptographically signs a prescription on-chain. This record is linked to a specific patient's public address and details the exact medication and dosage.
2. **Verification (Patient/Pharmacy):** A patient visits a pharmacy. The pharmacist queries the blockchain to verify the prescription's authenticity, ensuring it was issued by a valid doctor and hasn't been tampered with.
3. **Fulfillment (Pharmacy):** Once the medication is dispensed, the pharmacy authorizes a transaction that permanently marks the prescription as `fulfilled` on the blockchain. This prevents the patient from taking the same prescription to another pharmacy.

## ✨ Key Features 
* **Role-Based Authentication:** Utilizes Soroban's native `require_auth()` to ensure strict access control. Only authorized doctors can issue prescriptions, and only verified pharmacies can fulfill them.
* **Double-Fill Prevention:** Once a prescription's status transitions to `is_fulfilled = true`, the contract permanently locks it from being fulfilled a second time.
* **Immutable Patient Records:** Prescriptions are stored in Soroban's persistent state, providing a tamper-proof, auditable trail of a patient's medical history.
* **Cost-Efficient:** Built on Stellar, meaning transaction fees for creating and updating prescriptions are fractions of a cent, making it scalable for real-world healthcare infrastructure.

## 🚀 Deployed Smart Contract Link
* **Network:** Stellar
* **Contract ID:** `CBBH7TQTZITHOD3ODCCY55J4Q46OEOPZUU66UFGFJ4CW2LE52CJNOGIP`

---

## 🛠️ Tech Stack
* **Language:** [Rust](https://www.rust-lang.org/)
* **Framework:** [Soroban SDK](https://soroban.stellar.org/docs) `no_std` environment
* **Network:** [Stellar Blockchain](https://stellar.org/)

---

## 💻 Getting Started (For Developers)

### Prerequisites
To build and test this smart contract locally, you will need:
1. **Rust:** Install via [rustup](https://rustup.rs/)
2. **Target:** Add the WebAssembly target:
   ```bash
   rustup target add wasm32-unknown-unknown