use diesel::{Connection, PgConnection};
use dotenvy::dotenv;

pub mod data_model;
pub mod meetup_service;
pub mod schema;
pub mod user_service;

pub fn establish_db_connection() -> PgConnection {
    dotenv().ok();
    let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// TODO create macro to create service

pub trait ReadonlyService<T> {
    fn get(&mut self) -> Vec<T>;
}

pub trait CrudService<T, R>: ReadonlyService<R> {
    fn create(&mut self, new: &T) -> R;
}
