// use diesel::{Connection, PgConnection};
//
// use crate::lib::data_model::Meetup;
// use crate::lib::establish_db_connection;
//
// pub struct MeetupService {
//     connection: PgConnection,
// }
//
// impl MeetupService {
//     pub fn new() -> Self {
//         MeetupService {
//             connection: establish_db_connection(),
//         }
//     }
//
//     pub fn get(self) -> Vec<Meetup> {
//         meetups.load::<Meetup>(self.connection).unwrap()
//     }
// }
