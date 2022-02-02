-- Your SQL goes here

CREATE TABLE review(  
review_id INT GENERATED ALWAYS AS IDENTITY,  
review_details VARCHAR NOT NULL,
tour_plan_id INT NOT NULL,
location_id INT NOT NULL,
rating_count FlOAT NOT NULL,
PRIMARY KEY(review_id)  
);  