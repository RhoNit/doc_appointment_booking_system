CREATE TABLE appointments(
    id SERIAL PRIMARY KEY,
    patient_id INTEGER REFERENCES patients(id),
    doctor_id INTEGER REFERENCES doctors(id),
    time_slot VARCHAR NOT NULL,
    status VARCHAR NOT NULL
);