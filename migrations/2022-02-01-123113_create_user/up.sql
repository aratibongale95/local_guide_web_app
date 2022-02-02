-- Your SQL goes here

CREATE TABLE users(  
user_id INT GENERATED ALWAYS AS IDENTITY,  
user_name VARCHAR NOT NULL,
contact VARCHAR NOT NULL,
email VARCHAR NOT NULL,
password VARCHAR NOT NULL,
role_id INT NOT NULL,
location_id INT NOT NULL,
PRIMARY KEY(user_id)  
);  