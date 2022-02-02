-- Your SQL goes here

CREATE TABLE role(  
role_id INT GENERATED ALWAYS AS IDENTITY,  
role_name VARCHAR NOT NULL,

PRIMARY KEY(role_id)
);  