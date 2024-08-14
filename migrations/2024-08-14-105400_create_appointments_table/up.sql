CREATE TABLE appointments(
    id UUID PRIMARY KEY,
    doctor_id UUID REFERENCES doctors(id),
    patient_id UUID REFERENCES patients(id),
    appointment_status VARCHAR NOT NULL
);