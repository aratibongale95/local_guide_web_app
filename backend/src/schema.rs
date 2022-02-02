table! {
    appointment (appointment_id) {
        appointment_id -> Int4,
        appointment_details -> Varchar,
        appointment_date -> Date,
        user_id -> Int4,
        tour_plan_id -> Int4,
    }
}

table! {
    location (location_id) {
        location_id -> Int4,
        location_name -> Varchar,
        location_info -> Varchar,
    }
}

table! {
    review (review_id) {
        review_id -> Int4,
        review_details -> Varchar,
        tour_plan_id -> Int4,
        location_id -> Int4,
        rating_count -> Float8,
    }
}

table! {
    role (role_id) {
        role_id -> Int4,
        role_name -> Varchar,
    }
}

table! {
    tour_plan (tour_plan_id) {
        tour_plan_id -> Int4,
        tour_plan_details -> Varchar,
        tour_plan_cost -> Float8,
        user_id -> Int4,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        user_name -> Varchar,
        contact -> Varchar,
        email -> Varchar,
        password -> Varchar,
        role_id -> Int4,
        location_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    appointment,
    location,
    review,
    role,
    tour_plan,
    users,
);
