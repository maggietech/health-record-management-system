
# Health Record Management System

## Overview
This Rust program implements a simple Health Record Management System designed to store, retrieve, update, and delete health records. It also provides functionality to search for health records based on symptoms and diagnoses efficiently using indexes.

## Components

### Data Structures
- **HealthRecord**: Represents a health record containing fields such as ID, patient name, symptoms, diagnosis, treatment, creation timestamp, and optional update timestamp.
- **HealthRecordPayload**: A simplified version of `HealthRecord` used for adding or updating health records without the ID and timestamps.
- **Error**: An enum representing various error types that can occur during operations, including not found, validation error, and insertion failure.

### Storage
- **STORAGE**: A thread-local stable B-tree map storing health records keyed by their unique IDs.
- **SYMPTOMS_INDEX** and **DIAGNOSIS_INDEX**: Thread-local hash maps used for indexing health records by symptoms and diagnoses respectively. These indexes facilitate efficient searching.

### Memory Management
- **MEMORY_MANAGER**: Manages memory for the application.
- **ID_COUNTER**: Generates unique identifiers for health records.

### Functions
- **get_health_record**: Retrieves a health record by its ID.
- **search_by_symptom**: Searches for health records based on a given symptom.
- **search_by_diagnosis**: Searches for health records based on a given diagnosis.
- **add_health_record**: Adds a new health record to the system.
- **update_health_record**: Updates an existing health record.
- **delete_health_record**: Deletes a health record from the system.

### Helper Functions
- **do_insert**: Inserts a health record into the storage.
- **update_indexes**: Updates the symptom and diagnosis indexes.
- **log_event**: Logs events such as adding, updating, or deleting health records.

## Candid Interface
The program exports a Candid interface for interaction with the Internet Computer.

## Usage
1. Add a health record using `add_health_record`.
2. Retrieve a health record by its ID using `get_health_record`.
3. Search for health records by symptom or diagnosis using `search_by_symptom` or `search_by_diagnosis`.
4. Update an existing health record using `update_health_record`.
5. Delete a health record using `delete_health_record`.

## Error Handling
The program handles various error scenarios such as not found, validation errors, and insertion failures, providing informative error messages.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```