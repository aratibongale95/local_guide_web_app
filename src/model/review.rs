use crate::db;
use crate::error_handler::CustomError;
use crate::schema::review;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "review"]
pub struct Review {
    pub review_details: String,
    pub tour_plan_id: i32,
    pub location_id: i32,
    pub rating_count: f64,
}


#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "review"]
pub struct Reviews {
    pub review_id: i32,
    pub review_details: String,
    pub tour_plan_id: i32,
    pub location_id: i32,
    pub rating_count: f64,
   
}

impl Reviews {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let reviews = review::table.load::<Reviews>(&conn)?;
        Ok(reviews)
    }
    
    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let review = review::table.filter(review::review_id.eq(id)).first(&conn)?;
        Ok(review)
    }
     
    pub fn create(review: Review) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let review = Review::from(review);
        let review = diesel::insert_into(review::table)
            .values(review)
            .get_result(&conn)?;
        Ok(review)
    }
   

    pub fn update(id: i32, review: Review) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let review = diesel::update(review::table)
            .filter(review::review_id.eq(id))
            .set(review)
            .get_result(&conn)?;
        Ok(review)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(review::table.filter(review::review_id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
impl Review {
    fn from(review: Review) -> Review {
        Review {
            review_details: review.review_details,
            location_id: review.location_id,
            tour_plan_id: review.tour_plan_id,
            rating_count: review.rating_count,
        }
    }
}