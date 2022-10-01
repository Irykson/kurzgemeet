use std::error::Error;

use diesel::associations::HasTable;
use diesel::{Connection, PgConnection, RunQueryDsl};

use crate::lib::data_model::{NewEmailUser, NewPasswordUser, NewUser, User};
use crate::lib::establish_db_connection;
use crate::lib::schema::email_users::dsl::email_users;
use crate::lib::schema::password_users::dsl::password_users;
use crate::lib::schema::users::dsl::users;

pub struct CreateUser {
    pub name: String,
    pub password: String,
    pub email: Option<String>,
}

pub struct UserService {
    connection: PgConnection,
}

impl UserService {
    pub fn new() -> Self {
        UserService {
            connection: establish_db_connection(),
        }
    }
}

impl UserService {
    fn get(&mut self) -> Vec<User> {
        users.load::<User>(&mut self.connection).unwrap()
    }

    fn create(&mut self, new: CreateUser) -> User {
        let new_user = NewUser {
            name: new.name.clone(),
        };

        self.connection
            .transaction::<_, dyn Error, _>(|| {
                let new_user: User = diesel::insert_into(users::table)
                    .values(new_user)
                    .get_result(&mut self.connection)?;

                if let Some(email) = new.email {
                    let new_email = NewEmailUser {
                        user_id: new_user.id,
                        email,
                    };
                    diesel::insert_into(email_users::table)
                        .values(new_email)
                        .execute(&mut self.connection)?;
                }

                let new_password = NewPasswordUser {
                    user_id: new_user.id,
                    password: new.password,
                };
                diesel::insert_into(password_users::table)
                    .values(new_password)
                    .execute(&mut self.connection)?;

                Ok(new_user)
            })
            .unwrap()
    }
}
