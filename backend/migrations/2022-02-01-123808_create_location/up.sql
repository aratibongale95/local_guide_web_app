-- Your SQL goes here

CREATE TABLE location(  
location_id INT GENERATED ALWAYS AS IDENTITY,  
location_name VARCHAR NOT NULL,
location_info VARCHAR NOT NULL,

PRIMARY KEY(location_id)  
);  