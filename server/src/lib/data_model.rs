use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

use super::schema::*;

#[derive(Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
}

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
pub struct PasswordUser {
    id: i32,
    // TODO encrypt
    password: String,
    user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = password_users)]
pub struct NewPasswordUser {
    pub user_id: i32,
    pub password: String,
}

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
pub struct EmailUser {
    id: i32,
    email: String,
    user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = email_users)]
pub struct NewEmailUser {
    pub user_id: i32,
    pub email: String,
}

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Meetup))]
pub struct MeetupDate {
    id: i32,
    date: NaiveDateTime,
    meetup_id: i32,
}

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(MeetupDate))]
pub struct MeetupDateVote {
    id: i32,
    meetup_date_id: i32,
    user_id: i32,
}

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Meetup))]
pub struct MeetupPlace {
    id: i32,
    name: String,
    meetup_id: i32,
}

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(MeetupPlace))]
pub struct MeetupPlaceVote {
    id: i32,
    meetup_place_id: i32,
    user_id: i32,
}

#[derive(Queryable, Identifiable, Associations, Serialize)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(MeetupDate))]
#[diesel(belongs_to(MeetupPlace))]
pub struct Meetup {
    id: i32,
    password: String,
    name: String,
    description: String,
    user_id: i32,
    meetup_date_id: i32,
    meetup_place_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = meetups)]
pub struct NewMeetup {
    pub password: String,
    pub name: String,
    pub description: String,
    pub user_id: i32,
}
