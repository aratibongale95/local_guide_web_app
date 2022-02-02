use crate::db;
use crate::error_handler::CustomError;
use crate::schema::appointment;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::prelude::*; //For date and time
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "appointment"]
pub struct Appointment {
    pub appointment_details: String,
    pub appointment_date: NaiveDate,
    pub user_id: i32,
    pub tour_plan_id: i32,
}


#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "appointment"]
pub struct Appointments {
    pub appointment_id: i32,
    pub appointment_details: String,
    pub appointment_date: NaiveDate,
    pub user_id: i32,
    pub tour_plan_id: i32,
}

impl Appointments {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let appointments = appointment::table.load::<Appointments>(&conn)?;
        Ok(appointments)
    }
    
    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let appointment = appointment::table.filter(appointment::appointment_id.eq(id)).first(&conn)?;
        Ok(appointment)
    }

    pub fn create(appointment: Appointment) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let appointment = Appointment::from(appointment);
        let appointment = diesel::insert_into(appointment::table)
            .values(appointment)
            .get_result(&conn)?;
        Ok(appointment)
    }
     
    
    pub fn update(id: i32, appointment: Appointment) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let appointment = diesel::update(appointment::table)
            .filter(appointment::appointment_id.eq(id))
            .set(appointment)
            .get_result(&conn)?;
        Ok(appointment)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(appointment::table.filter(appointment::appointment_id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
impl Appointment {
    fn from(appointment: Appointment) -> Appointment {
        
        Appointment {

            appointment_details: appointment.appointment_details,
            appointment_date: appointment.appointment_date,
            user_id: appointment.user_id,
            tour_plan_id: appointment.tour_plan_id,
            
        }
    }
}