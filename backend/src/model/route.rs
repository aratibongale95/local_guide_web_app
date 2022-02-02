use crate::model::{User, Users};
use crate::model::{Role,Roles};
use crate::model::{Location,Locations};
use crate::model::{Review,Reviews};
use crate::model::{TourPlan,TourPlans};
use crate::model::{Appointment,Appointments};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use actix_cors::Cors;

//user routes

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


// role routes

#[get("/roles")]
async fn find_all_roles() -> Result<HttpResponse, CustomError> {
    let roles = Roles::find_all()?;
    Ok(HttpResponse::Ok().json(roles))
}

#[get("/roles/{id}")]
async fn find_role_by_id(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let role = Roles::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(role))
}

#[post("/roles")]
async fn create_role(role: web::Json<Role>) -> Result<HttpResponse, CustomError> {
    let role = Roles::create(role.into_inner())?;
    Ok(HttpResponse::Ok().json(role))
}


#[put("/roles/{id}")]
async fn update_role(
    id: web::Path<i32>,
    role: web::Json<Role>,
) -> Result<HttpResponse, CustomError> {
    let role = Roles::update(id.into_inner(), role.into_inner())?;
    Ok(HttpResponse::Ok().json(role))
}


#[delete("/roles/{id}")]
async fn delete_role(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_role = Roles::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_role })))
}

// location routes

#[get("/locations")]
async fn find_all_locations() -> Result<HttpResponse, CustomError> {
    let locations = Locations::find_all()?;
    Ok(HttpResponse::Ok().json(locations))
}

#[get("/locations/{id}")]
async fn find_location_by_id(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let location = Locations::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(location))
}

#[post("/locations")]
async fn create_location(location: web::Json<Location>) -> Result<HttpResponse, CustomError> {
    let location = Locations::create(location.into_inner())?;
    Ok(HttpResponse::Ok().json(location))
}


#[put("/locations/{id}")]
async fn update_location(
    id: web::Path<i32>,
    location: web::Json<Location>,
) -> Result<HttpResponse, CustomError> {
    let location = Locations::update(id.into_inner(), location.into_inner())?;
    Ok(HttpResponse::Ok().json(location))
}


#[delete("/locations/{id}")]
async fn delete_location(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_location = Locations::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_location })))
}

// review routes

#[get("/reviews")]
async fn find_all_reviews() -> Result<HttpResponse, CustomError> {
    let reviews = Reviews::find_all()?;
    Ok(HttpResponse::Ok().json(reviews))
}

#[get("/reviews/{id}")]
async fn find_review_by_id(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let review = Reviews::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(review))
}

#[post("/reviews")]
async fn create_review(review: web::Json<Review>) -> Result<HttpResponse, CustomError> {
    let review = Reviews::create(review.into_inner())?;
    Ok(HttpResponse::Ok().json(review))
}


#[put("/reviews/{id}")]
async fn update_review(
    id: web::Path<i32>,
    review: web::Json<Review>,
) -> Result<HttpResponse, CustomError> {
    let review = Reviews::update(id.into_inner(), review.into_inner())?;
    Ok(HttpResponse::Ok().json(review))
}


#[delete("/reviews/{id}")]
async fn delete_review(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_review = Reviews::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_review })))
}

//tour plan routes


#[get("/tour_plans")]
async fn find_all_tour_plans() -> Result<HttpResponse, CustomError> {
    let tour_plans = TourPlans::find_all()?;
    Ok(HttpResponse::Ok().json(tour_plans))
}

#[get("/tour_plans/{id}")]
async fn find_tour_plan_by_id(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let tour_plan = TourPlans::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(tour_plan))
}

#[post("/tour_plans")]
async fn create_tour_plan(tour_plan: web::Json<TourPlan>) -> Result<HttpResponse, CustomError> {
    let tour_plan = TourPlans::create(tour_plan.into_inner())?;
    Ok(HttpResponse::Ok().json(tour_plan))
}


#[put("/tour_plans/{id}")]
async fn update_tour_plan(
    id: web::Path<i32>,
    tour_plan: web::Json<TourPlan>,
) -> Result<HttpResponse, CustomError> {
    let tour_plan = TourPlans::update(id.into_inner(), tour_plan.into_inner())?;
    Ok(HttpResponse::Ok().json(tour_plan))
}


#[delete("/tour_plans/{id}")]
async fn delete_tour_plan(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_tour_plan = TourPlans::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_tour_plan })))
}


// appointment routes


#[get("/appointments")]
async fn find_all_appointments() -> Result<HttpResponse, CustomError> {
    let appointments = Appointments::find_all()?;
    Ok(HttpResponse::Ok().json(appointments))
}

#[get("/appointments/{id}")]
async fn find_appointment_by_id(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let appointment = Appointments::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(appointment))
}

#[post("/appointments")]
async fn create_appointment(appointment: web::Json<Appointment>) -> Result<HttpResponse, CustomError> {
    let appointment = Appointments::create(appointment.into_inner())?;
    Ok(HttpResponse::Ok().json(appointment))
}


#[put("/appointments/{id}")]
async fn update_appointment(
    id: web::Path<i32>,
    appointment: web::Json<Appointment>,
) -> Result<HttpResponse, CustomError> {
    let appointment = Appointments::update(id.into_inner(), appointment.into_inner())?;
    Ok(HttpResponse::Ok().json(appointment))
}


#[delete("/appointments/{id}")]
async fn delete_appointment(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_appointment = Appointments::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_appointment })))
}


pub fn init_routes(config: &mut web::ServiceConfig) {

    config.service(find_all_users);
    config.service(find_user_by_id);
    config.service(create_user);
    config.service(update_user);
    config.service(delete_user);


    config.service(find_all_roles);
    config.service(find_role_by_id);
    config.service(create_role);
    config.service(update_role);
    config.service(delete_role);


    config.service(find_all_locations);
    config.service(find_location_by_id);
    config.service(create_location);
    config.service(update_location);
    config.service(delete_location);


    config.service(find_all_reviews);
    config.service(find_review_by_id);
    config.service(create_review);
    config.service(update_review);
    config.service(delete_review);


    config.service(find_all_tour_plans);
    config.service(find_tour_plan_by_id);
    config.service(create_tour_plan);
    config.service(update_tour_plan);
    config.service(delete_tour_plan);


    config.service(find_all_appointments);
    config.service(find_appointment_by_id);
    config.service(create_appointment);
    config.service(update_appointment);
    config.service(delete_appointment);

}