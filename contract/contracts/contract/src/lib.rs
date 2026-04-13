#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Prescription {
    pub doctor: Address,
    pub patient: Address,
    pub medication: String,
    pub dosage: String,
    pub is_fulfilled: bool,
}

#[contract]
pub struct PrescriptionChain;

#[contractimpl]
impl PrescriptionChain {
    /// Creates a new prescription on the ledger.
    /// Only the `doctor` Address can authorize this action.
    pub fn create_rx(
        env: Env,
        rx_id: u64,
        doctor: Address,
        patient: Address,
        medication: String,
        dosage: String,
    ) {
        // Require the doctor's cryptographic signature to create this record
        doctor.require_auth();

        // Ensure the prescription ID doesn't already exist
        if env.storage().persistent().has(&rx_id) {
            panic!("Prescription ID already exists");
        }

        let prescription = Prescription {
            doctor,
            patient,
            medication,
            dosage,
            is_fulfilled: false,
        };

        // Save the prescription to persistent storage
        env.storage().persistent().set(&rx_id, &prescription);
    }

    /// Marks a prescription as fulfilled.
    /// In a real-world scenario, you might restrict this to whitelisted 'Pharmacy' addresses.
    pub fn fulfill_rx(env: Env, rx_id: u64, pharmacy: Address) {
        // Require the pharmacy to authorize the fulfillment
        pharmacy.require_auth();

        // Fetch the prescription
        let mut prescription: Prescription = env
            .storage()
            .persistent()
            .get(&rx_id)
            .expect("Prescription not found");

        if prescription.is_fulfilled {
            panic!("Prescription has already been fulfilled");
        }

        // Update status and save back to storage
        prescription.is_fulfilled = true;
        env.storage().persistent().set(&rx_id, &prescription);
    }

    /// Retrieves the details of a specific prescription.
    pub fn get_rx(env: Env, rx_id: u64) -> Prescription {
        env.storage()
            .persistent()
            .get(&rx_id)
            .expect("Prescription not found")
    }
}