// @generated automatically by Diesel CLI.

diesel::table! {
    appointments (id) {
        id -> Int4,
        patient_id -> Nullable<Int4>,
        doctor_id -> Nullable<Int4>,
        time_slot -> Varchar,
        status -> Varchar,
    }
}

diesel::table! {
    doctors (id) {
        id -> Int4,
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
        id -> Int4,
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
