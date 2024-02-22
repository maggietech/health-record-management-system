export const idlFactory = ({ IDL }) => {
  const HealthRecordPayload = IDL.Record({
    'patient_name' : IDL.Text,
    'treatment' : IDL.Text,
    'diagnosis' : IDL.Text,
    'symptoms' : IDL.Text,
  });
  const HealthRecord = IDL.Record({
    'id' : IDL.Nat64,
    'patient_name' : IDL.Text,
    'updated_at' : IDL.Opt(IDL.Nat64),
    'treatment' : IDL.Text,
    'created_at' : IDL.Nat64,
    'diagnosis' : IDL.Text,
    'symptoms' : IDL.Text,
  });
  const Error = IDL.Variant({
    'NotFound' : IDL.Record({ 'msg' : IDL.Text }),
    'ValidationError' : IDL.Text,
  });
  const Result = IDL.Variant({ 'Ok' : HealthRecord, 'Err' : Error });
  return IDL.Service({
    'add_health_record' : IDL.Func(
        [HealthRecordPayload],
        [IDL.Opt(HealthRecord)],
        [],
      ),
    'delete_health_record' : IDL.Func([IDL.Nat64], [Result], []),
    'get_health_record' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'search_by_diagnosis' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(HealthRecord)],
        ['query'],
      ),
    'search_by_symptom' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(HealthRecord)],
        ['query'],
      ),
    'update_health_record' : IDL.Func(
        [IDL.Nat64, HealthRecordPayload],
        [Result],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
