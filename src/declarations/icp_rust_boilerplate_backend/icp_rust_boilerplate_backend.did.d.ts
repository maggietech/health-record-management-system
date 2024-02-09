import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type Error = { 'InsertionFailed' : string } |
  { 'NotFound' : { 'msg' : string } } |
  { 'ValidationError' : string };
export interface HealthRecord {
  'id' : bigint,
  'patient_name' : string,
  'updated_at' : [] | [bigint],
  'treatment' : string,
  'created_at' : bigint,
  'diagnosis' : string,
  'symptoms' : string,
}
export interface HealthRecordPayload {
  'patient_name' : string,
  'treatment' : string,
  'diagnosis' : string,
  'symptoms' : string,
}
export type Result = { 'Ok' : HealthRecord } |
  { 'Err' : Error };
export interface _SERVICE {
  'add_health_record' : ActorMethod<[HealthRecordPayload], Result>,
  'delete_health_record' : ActorMethod<[bigint], Result>,
  'get_health_record' : ActorMethod<[bigint], Result>,
  'search_by_diagnosis' : ActorMethod<[string], Array<HealthRecord>>,
  'search_by_symptom' : ActorMethod<[string], Array<HealthRecord>>,
  'update_health_record' : ActorMethod<[bigint, HealthRecordPayload], Result>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: ({ IDL }: { IDL: IDL }) => IDL.Type[];
