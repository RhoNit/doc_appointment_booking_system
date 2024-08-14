CREATE TABLE doctors(
    id SERIAL PRIMARY KEY,
    doctor_name VARCHAR NOT NULL,
    registration_number VARCHAR NOT NULL,
    speciality VARCHAR NOT NULL,
    availabilty VARCHAR NOT NULL,
    phone VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    city VARCHAR NOT NULL
);