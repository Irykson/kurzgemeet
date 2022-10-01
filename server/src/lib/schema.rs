// @generated automatically by Diesel CLI.

diesel::table! {
    email_users (id) {
        id -> Int4,
        email -> Varchar,
        user_id -> Int4,
    }
}

diesel::table! {
    meetup_date_votes (id) {
        id -> Int4,
        meetup_date_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    meetup_dates (id) {
        id -> Int4,
        date -> Date,
        meetup_id -> Int4,
    }
}

diesel::table! {
    meetup_place_votes (id) {
        id -> Int4,
        meetup_place_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    meetup_places (id) {
        id -> Int4,
        place -> Varchar,
        meetup_id -> Int4,
    }
}

diesel::table! {
    meetups (id) {
        id -> Int4,
        password -> Varchar,
        name -> Varchar,
        description -> Varchar,
        user_id -> Int4,
        meetup_date_id -> Int4,
        meetup_place_id -> Int4,
    }
}

diesel::table! {
    password_users (id) {
        id -> Int4,
        password -> Varchar,
        user_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(email_users -> users (user_id));
diesel::joinable!(meetup_date_votes -> meetup_dates (meetup_date_id));
diesel::joinable!(meetup_date_votes -> users (user_id));
diesel::joinable!(meetup_place_votes -> meetup_places (meetup_place_id));
diesel::joinable!(meetup_place_votes -> users (user_id));
diesel::joinable!(meetups -> users (user_id));
diesel::joinable!(password_users -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    email_users,
    meetup_date_votes,
    meetup_dates,
    meetup_place_votes,
    meetup_places,
    meetups,
    password_users,
    users,
);
