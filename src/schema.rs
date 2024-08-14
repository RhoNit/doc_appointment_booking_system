// @generated automatically by Diesel CLI.

diesel::table! {
    appointments (id) {
        id -> Uuid,
        doctor_id -> Nullable<Uuid>,
        patient_id -> Nullable<Uuid>,
        appointment_status -> Varchar,
    }
}

diesel::table! {
    doctors (id) {
        id -> Uuid,
        doctor_name -> Varchar,
        registration_number -> Varchar,
        speciality -> Varchar,
        availabilty -> Varchar,
        phone -> Varchar,
        email -> Varchar,
        city -> Varchar,
    }
}

diesel::table! {
    patients (id) {
        id -> Uuid,
        patient_name -> Varchar,
        age -> Int4,
        phone -> Varchar,
        email -> Nullable<Varchar>,
        city -> Varchar,
    }
}

diesel::joinable!(appointments -> doctors (doctor_id));
diesel::joinable!(appointments -> patients (patient_id));

diesel::allow_tables_to_appear_in_same_query!(
    appointments,
    doctors,
    patients,
);
