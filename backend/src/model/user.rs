use crate::db;
use crate::error_handler::CustomError;
use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct User {
    pub user_name: String,
    pub password: String,
    pub email: String,
    pub contact: String,
    pub role_id: i32,
    pub location_id: i32,
}


#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct Users {
    pub user_id: i32,
    pub user_name: String,
    pub password: String,
    pub email: String,
    pub contact: String,
    pub role_id: i32,
    pub location_id: i32,
}

impl Users {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let users = users::table.load::<Users>(&conn)?;
        Ok(users)
    }
    
    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let user = users::table.filter(users::user_id.eq(id)).first(&conn)?;
        Ok(user)
    }
     
    pub fn create(user: User) -> Result<Self, CustomError> {
        let hashed_pwd = hash(&user.password, DEFAULT_COST).unwrap();
        let conn = db::connection()?;
        let user = User{
            password: hashed_pwd,
            ..user
        };
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&conn)?;
        Ok(user)
    }

    pub fn update(id: i32, user: User) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let user = diesel::update(users::table)
            .filter(users::user_id.eq(id))
            .set(user)
            .get_result(&conn)?;
        Ok(user)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(users::table.filter(users::user_id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl User {
    fn from(user: User) -> User {
        User {
            user_name: user.user_name,
            password: user.password,
            email: user.email,
            contact: user.contact,
            role_id: user.role_id,
            location_id: user.location_id,
        }
    }
}