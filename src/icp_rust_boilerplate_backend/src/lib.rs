#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
use std::collections::HashMap;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Define HashMaps for indexing
thread_local! {
    static SYMPTOMS_INDEX: RefCell<HashMap<String, Vec<u64>>> = RefCell::new(HashMap::new());
    static DIAGNOSIS_INDEX: RefCell<HashMap<String, Vec<u64>>> = RefCell::new(HashMap::new());
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct HealthRecord {
    id: u64,
    patient_name: String,
    symptoms: String,
    diagnosis: String,
    treatment: String,
    created_at: u64,
    updated_at: Option<u64>,
}

// a trait that must be implemented for a struct that is stored in a stable struct
impl Storable for HealthRecord {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// another trait that must be implemented for a struct that is stored in a stable struct
impl BoundedStorable for HealthRecord {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static STORAGE: RefCell<StableBTreeMap<u64, HealthRecord, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct HealthRecordPayload {
    patient_name: String,
    symptoms: String,
    diagnosis: String,
    treatment: String,
}

#[ic_cdk::query]
fn get_health_record(id: u64) -> Result<HealthRecord, Error> {
    match _get_health_record(&id) {
        Some(record) => Ok(record),
        None => Err(Error::NotFound {
            msg: format!("a health record with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn search_by_symptom(symptom: String) -> Vec<HealthRecord> {
    let records = SYMPTOMS_INDEX.with(|index| {
        let index_map = index.borrow();
        if let Some(ids) = index_map.get(&symptom) {
            ids.iter().filter_map(|&id| _get_health_record(&id)).collect()
        } else {
            Vec::new()
        }
    });
    records
}

#[ic_cdk::query]
fn search_by_diagnosis(diagnosis: String) -> Vec<HealthRecord> {
    let records = DIAGNOSIS_INDEX.with(|index| {
        let index_map = index.borrow();
        if let Some(ids) = index_map.get(&diagnosis) {
            ids.iter().filter_map(|&id| _get_health_record(&id)).collect()
        } else {
            Vec::new()
        }
    });
    records
}

#[ic_cdk::update]
fn add_health_record(record: HealthRecordPayload) -> Result<HealthRecord, Error> {
    // Perform data validation
    if record.patient_name.is_empty() {
        return Err(Error::ValidationError("Patient name cannot be empty".to_string()));
    }
    if record.symptoms.is_empty() {
        return Err(Error::ValidationError("Symptoms cannot be empty".to_string()));
    }
    if record.diagnosis.is_empty() {
        return Err(Error::ValidationError("Diagnosis cannot be empty".to_string()));
    }
    if record.treatment.is_empty() {
        return Err(Error::ValidationError("Treatment cannot be empty".to_string()));
    }
    
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let record = HealthRecord {
        id,
        patient_name: record.patient_name,
        symptoms: record.symptoms.clone(),
        diagnosis: record.diagnosis.clone(),
        treatment: record.treatment,
        created_at: time(),
        updated_at: None,
    };
    // Insert record into STORAGE
    do_insert(&record)?;
    // Update indexes
    update_indexes(&record);
    // Log the event
    log_event("add_health_record", id);
    Ok(record)
}

#[ic_cdk::update]
fn update_health_record(id: u64, payload: HealthRecordPayload) -> Result<HealthRecord, Error> {
    // Perform data validation
    if payload.patient_name.is_empty() {
        return Err(Error::ValidationError("Patient name cannot be empty".to_string()));
    }
    if payload.symptoms.is_empty() {
        return Err(Error::ValidationError("Symptoms cannot be empty".to_string()));
    }
    if payload.diagnosis.is_empty() {
        return Err(Error::ValidationError("Diagnosis cannot be empty".to_string()));
    }
    if payload.treatment.is_empty() {
        return Err(Error::ValidationError("Treatment cannot be empty".to_string()));
    }
    
    match STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut record) => {
            record.patient_name = payload.patient_name;
            record.symptoms = payload.symptoms;
            record.diagnosis = payload.diagnosis;
            record.treatment = payload.treatment;
            record.updated_at = Some(time());
            do_insert(&record)?;
            // Log the event
            log_event("update_health_record", id);
            Ok(record)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update a health record with id={}. Record not found",
                id
            ),
        }),
    }
}

fn do_insert(record: &HealthRecord) -> Result<(), Error> {
    STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(record.id, record.clone())
            .map(|_| ())
            .ok_or(Error::InsertionFailed("Failed to insert health record".to_string()))
    })
}

#[ic_cdk::update]
fn delete_health_record(id: u64) -> Result<HealthRecord, Error> {
    match STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(record) => {
            // Log the event
            log_event("delete_health_record", id);
            Ok(record)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete a health record with id={}. Record not found.",
                id
            ),
        }),
    }
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    ValidationError(String),
    InsertionFailed(String),
}

fn _get_health_record(id: &u64) -> Option<HealthRecord> {
    STORAGE.with(|service| service.borrow().get(id))
}

fn update_indexes(record: &HealthRecord) {
    SYMPTOMS_INDEX.with(|index| {
        let mut index_map = index.borrow_mut();
        for symptom in record.symptoms.split(',') {
            let ids = index_map.entry(symptom.to_string()).or_insert_with(Vec::new);
            ids.push(record.id);
        }
    });

    DIAGNOSIS_INDEX.with(|index| {
        let mut index_map = index.borrow_mut();
        for diagnosis in record.diagnosis.split(',') {
            let ids = index_map.entry(diagnosis.to_string()).or_insert_with(Vec::new);
            ids.push(record.id);
        }
    });
}

// Helper method to log events
fn log_event(event_type: &str, record_id: u64) {
    println!("Event logged: Type={}, Record ID={}", event_type, record_id);
}

// need this to generate candid
ic_cdk::export_candid!();
