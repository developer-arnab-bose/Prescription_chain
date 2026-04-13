# 💊 Prescription Chain

A decentralized healthcare application built on the Stellar network using Soroban smart contracts to securely manage medical prescriptions.

-----

![Image](https://raw.githubusercontent.com/developer-arnab-bose/Prescription_chain/refs/heads/main/contract/screenshot.png)

## 🌍 Overview

Prescription Chain is a lightweight, on-chain platform that enables doctors to issue, patients to hold, and pharmacies to fulfill prescriptions without relying on fragmented, centralized healthcare databases.

It brings the concept of secure medical records to the blockchain — ensuring transparency, preventing prescription fraud, and coordinating securely between healthcare participants.

-----

## ✨ Key Highlights

  * ⚡ Fully on-chain prescription management
  * 🔐 Secure authorization via Stellar accounts
  * 🧱 Built using Soroban smart contracts (Rust)
  * 🛑 Double-fill prevention mechanism
  * 🌐 Ready for healthcare dApp integration

-----

## 🧠 How It Works

1.  **Issue Prescription**
    A licensed Doctor creates and signs a prescription on-chain with medication details.

2.  **Verify Prescription**
    A Pharmacist queries the blockchain to verify the prescription's authenticity and ensure it hasn't been altered.

3.  **Fulfill Prescription**
    The Pharmacy marks the prescription as completed once the medication is dispensed.

4.  **Track On-Chain**
    All fulfillment data is stored transparently and permanently on the blockchain.

-----

## 🛠️ Features

### Core Functionality

  * 📝 Create prescriptions with unique IDs
  * 🩺 Assign specific medications and dosages
  * ✅ Mark prescriptions as fulfilled
  * 🔍 Retrieve prescription details anytime
  * 🔐 Enforced authentication (`require_auth`)

-----

### 🧱 Smart Contract Design

#### `Prescription` Structure

| Field          | Type    | Description             |
| -------------- | ------- | ----------------------- |
| `doctor`       | Address | Prescribing doctor      |
| `patient`      | Address | Receiving patient       |
| `medication`   | String  | Name of the drug        |
| `dosage`       | String  | Dosage instructions     |
| `is_fulfilled` | bool    | Fulfillment status      |

-----

### ⚙️ Contract Functions

| Function     | Description                        |
| ------------ | ---------------------------------- |
| `create_rx`  | Create a new prescription          |
| `fulfill_rx` | Mark prescription as dispensed     |
| `get_rx`     | Retrieve prescription details      |

-----

## 🌐 Deployed Contract

🔗 **Contract Address:**
`CAHTKZIENCHISUC72EFG3T3P4NENFIVQNEFJDWCJ7AZOO7SQTBGK5Y2W`

DEPLOYED LINK
[https://stellar.expert/explorer/testnet/contract/CBBH7TQTZITHOD3ODCCY55J4Q46OEOPZUU66UFGFJ4CW2LE52CJNOGIP](https://www.google.com/search?q=https://stellar.expert/explorer/testnet/contract/CBBH7TQTZITHOD3ODCCY55J4Q46OEOPZUU66UFGFJ4CW2LE52CJNOGIP)

👉 You can interact with it using the Soroban CLI or Stellar tools.

-----

## 🧪 Example Usage

### Create a Prescription

```bash id="mkd82n"
soroban contract invoke \
  --id CAHTKZIENCHISUC72EFG3T3P4NENFIVQNEFJDWCJ7AZOO7SQTBGK5Y2W \
  --fn create_rx \
  --arg env=... \
  --arg rx_id=101 \
  --arg doctor=<DOCTOR_ADDRESS> \
  --arg patient=<PATIENT_ADDRESS> \
  --arg medication="Amoxicillin" \
  --arg dosage="500mg twice daily"
```

-----

### Fulfill a Prescription

```bash id="8slx3k"
soroban contract invoke \
  --id CAHTKZIENCHISUC72EFG3T3P4NENFIVQNEFJDWCJ7AZOO7SQTBGK5Y2W \
  --fn fulfill_rx \
  --arg rx_id=101 \
  --arg pharmacy=<PHARMACY_ADDRESS>
```

-----

### Get Prescription Details

```bash id="a9v2c1"
soroban contract invoke \
  --id CAHTKZIENCHISUC72EFG3T3P4NENFIVQNEFJDWCJ7AZOO7SQTBGK5Y2W \
  --fn get_rx \
  --arg rx_id=101
```

-----

## 🧰 Tech Stack

  * **Smart Contract:** Rust + Soroban SDK
  * **Blockchain:** Stellar (Soroban)
  * **Storage:** On-chain persistent storage

-----

## 🔐 Security Model

  * ✅ Only authorized doctors can create prescriptions
  * ✅ Only authorized pharmacies can fulfill prescriptions
  * ✅ Prevents double-filling (locked once `is_fulfilled` is true)
  * ✅ Immutable patient records

-----

## ⚠️ Current Limitations

  * ❌ No privacy/encryption logic yet (requires ZK or off-chain encryption for HIPAA compliance)
  * ❌ No frontend interface
  * ❌ No global registry of verified Doctor/Pharmacy addresses
  * ❌ No insurance/payment integration

-----

## 🔮 Future Roadmap

  * 🔒 Encrypt patient data and medication details on-chain
  * 🌐 Build React frontend for doctors and pharmacies
  * ⭐ Global registry for verified healthcare providers
  * 💳 Integrate USDC payments for medication costs
  * 📣 Event logging for real-time pharmacy notifications

-----

## 🏗️ Getting Started (Dev)

```bash id="k2pz8w"
# Build contract
soroban contract build

# Deploy contract
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/contract.wasm
```

-----

## 🤝 Contributing

Pull requests are welcome\!
Feel free to open issues for suggestions or improvements.

-----

## 📄 License

MIT License

-----

## 💡 Vision

To create a fully decentralized, fraud-free healthcare ecosystem where patients truly own their medical records and prescriptions are globally verifiable — without data silos or intermediaries.
