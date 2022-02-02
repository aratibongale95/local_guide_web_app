-- Your SQL goes here

CREATE TABLE appointment(  
appointment_id INT GENERATED ALWAYS AS IDENTITY,  
appointment_details VARCHAR NOT NULL,
appointment_date DATE NOT NULL,
user_id INT NOT NULL,
tour_plan_id INT NOT NULL,
PRIMARY KEY(appointment_id)  
);  