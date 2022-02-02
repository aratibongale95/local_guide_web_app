use crate::db;
use crate::error_handler::CustomError;
use crate::schema::location;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "location"]
pub struct Location {
    pub location_name: String,
    pub location_info: String,
}


#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "location"]
pub struct Locations {
    pub location_id: i32,
    pub location_name: String,
    pub location_info: String,
}

impl Locations {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let locations = location::table.load::<Locations>(&conn)?;
        Ok(locations)
    }
    
    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let location = location::table.filter(location::location_id.eq(id)).first(&conn)?;
        Ok(location)
    }
     
    pub fn create(location: Location) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let location = Location::from(location);
        let location = diesel::insert_into(location::table)
            .values(location)
            .get_result(&conn)?;
        Ok(location)
    }

    pub fn update(id: i32, location: Location) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let location = diesel::update(location::table)
            .filter(location::location_id.eq(id))
            .set(location)
            .get_result(&conn)?;
        Ok(location)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(location::table.filter(location::location_id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
impl Location {
    fn from(location: Location) -> Location {
        
        Location {

            location_name: location.location_name,
            location_info: location.location_info,
            
        }
    }
}