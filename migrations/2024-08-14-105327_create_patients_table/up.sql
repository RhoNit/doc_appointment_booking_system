CREATE TABLE patients(
    id UUID PRIMARY KEY,
    patient_name VARCHAR NOT NULL,
    age INT NOT NULL,
    phone VARCHAR NOT NULL,
    email VARCHAR,
    city VARCHAR NOT NULL
);