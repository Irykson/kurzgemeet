use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::PgConnection;

use crate::lib::data_model::{Meetup, NewMeetup};
use crate::lib::schema::meetups;
use crate::lib::schema::meetups::dsl::*;
use crate::lib::{establish_db_connection, CrudService, ReadonlyService};

pub struct MeetupService {
    connection: PgConnection,
}

impl MeetupService {
    pub fn new() -> Self {
        MeetupService {
            connection: establish_db_connection(),
        }
    }
}

impl ReadonlyService<Meetup> for MeetupService {
    fn get(&mut self) -> Vec<Meetup> {
        meetups.load::<Meetup>(&mut self.connection).unwrap()
    }
}

impl CrudService<NewMeetup, Meetup> for MeetupService {
    fn create(&mut self, new: &NewMeetup) -> Meetup {
        diesel::insert_into(meetups::table)
            .values(new)
            .get_result(&mut self.connection)
            .unwrap()
    }
}
