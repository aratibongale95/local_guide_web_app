use crate::model::{User, Users};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use actix_cors::Cors;


#[get("/users")]
async fn find_all_users() -> Result<HttpResponse, CustomError> {
    let users = Users::find_all()?;
    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{id}")]
async fn find_user_by_id(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let user = Users::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
async fn create_user(user: web::Json<User>) -> Result<HttpResponse, CustomError> {
    let user = Users::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}


#[put("/users/{id}")]
async fn update_user(
    id: web::Path<i32>,
    user: web::Json<User>,
) -> Result<HttpResponse, CustomError> {
    let user = Users::update(id.into_inner(), user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}


#[delete("/users/{id}")]
async fn delete_user(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_user = Users::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_user })))
}





pub fn init_routes(config: &mut web::ServiceConfig) {

    config.service(find_all_users);
    config.service(find_user_by_id);
    config.service(create_user);
    config.service(update_user);
    config.service(delete_user);

}