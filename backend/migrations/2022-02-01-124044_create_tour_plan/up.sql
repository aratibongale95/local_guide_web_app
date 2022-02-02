-- Your SQL goes here

CREATE TABLE tour_plan(  
tour_plan_id INT GENERATED ALWAYS AS IDENTITY,  
tour_plan_details VARCHAR NOT NULL,
tour_plan_cost FlOAT NOT NULL,
user_id INT NOT NULL,

PRIMARY KEY(tour_plan_id)  
);  