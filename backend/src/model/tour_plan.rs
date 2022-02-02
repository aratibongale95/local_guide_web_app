use crate::db;
use crate::error_handler::CustomError;
use crate::schema::tour_plan;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "tour_plan"]
pub struct TourPlan {
    pub tour_plan_details: String,
    pub tour_plan_cost: f64,
    pub user_id: i32,
   
    
}


#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "tour_plan"]
pub struct TourPlans {
    pub tour_plan_id: i32,
    pub tour_plan_details: String,
    pub tour_plan_cost: f64,
    pub user_id: i32,
}

impl TourPlans {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let tour_plans = tour_plan::table.load::<TourPlans>(&conn)?;
        Ok(tour_plans)
    }
    
    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let tour_plan = tour_plan::table.filter(tour_plan::tour_plan_id.eq(id)).first(&conn)?;
        Ok(tour_plan)
    }
     
    pub fn create(tour_plan: TourPlan) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let tour_plan = TourPlan::from(tour_plan);
        let tour_plan = diesel::insert_into(tour_plan::table)
            .values(tour_plan)
            .get_result(&conn)?;
        Ok(tour_plan)
    }

    pub fn update(id: i32, tour_plan: TourPlan) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let tour_plan = diesel::update(tour_plan::table)
            .filter(tour_plan::tour_plan_id.eq(id))
            .set(tour_plan)
            .get_result(&conn)?;
        Ok(tour_plan)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(tour_plan::table.filter(tour_plan::tour_plan_id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
impl TourPlan {
    fn from(tour_plan: TourPlan) -> TourPlan {

        TourPlan {
            
            tour_plan_cost: tour_plan.tour_plan_cost,
            tour_plan_details: tour_plan.tour_plan_details,
            user_id: tour_plan.user_id,
        }
    }
}