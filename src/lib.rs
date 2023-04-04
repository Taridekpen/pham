use near_sdk::{near_bindgen, env};
use std::collections::HashMap;
use borsh::{BorshDeserialize, BorshSerialize};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Pharmacy {
    patients: HashMap<String, PatientData>,
    last_patient_id: u64,
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Clone)]
pub struct PatientData {
    name: String,
    age: u8,
    address: String,
    phone_number: String,
    diagnosis: String,
}

impl Pharmacy {
    pub fn register_patient(&mut self, name: String, age: u8, address: String, phone_number: String, diagnosis: String) -> String {
        let patient_id = format!("patient_{}", self.last_patient_id + 1);
        self.patients.insert(patient_id.clone(), PatientData {
            name,
            age,
            address,
            phone_number,
            diagnosis,
        });
        self.last_patient_id += 1;
        env::log_str(std::str::from_utf8(format!("Patient registered with ID: {}", patient_id).as_bytes()).unwrap());
        patient_id
    }

    pub fn get_patient_data(&self, patient_id: String) -> Option<PatientData> {
        self.patients.get(&patient_id).cloned()
    }
}
