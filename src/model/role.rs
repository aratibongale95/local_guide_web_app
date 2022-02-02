use crate::db;
use crate::error_handler::CustomError;
use crate::schema::role;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "role"]
pub struct Role {
    pub role_name: String,
   
}


#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "role"]
pub struct Roles {
    pub role_id: i32,
    pub role_name: String,
}

impl Roles {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let roles = role::table.load::<Roles>(&conn)?;
        Ok(roles)
    }
    
    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let role = role::table.filter(role::role_id.eq(id)).first(&conn)?;
        Ok(role)
    }
     
    pub fn create(role: Role) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let role = Role::from(role);
        let role = diesel::insert_into(role::table)
            .values(role)
            .get_result(&conn)?;
        Ok(role)
    }
    

    pub fn update(id: i32, role: Role) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let role = diesel::update(role::table)
            .filter(role::role_id.eq(id))
            .set(role)
            .get_result(&conn)?;
        Ok(role)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(role::table.filter(role::role_id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
impl Role {
    fn from(role: Role) -> Role {
        Role {
            role_name: role.role_name,
        }
    }
}