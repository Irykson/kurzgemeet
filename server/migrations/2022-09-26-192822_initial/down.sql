-- This file should undo anything in `up.sql`
drop table if exists password_users;
drop table if exists email_users;
alter table if exists meetups
    drop column meetup_date_id;
alter table if exists meetups
    drop column meetup_place_id;

drop table if exists meetup_date_votes;
drop table if exists meetup_place_votes;
drop table if exists meetup_dates;
drop table if exists meetup_places;
drop table if exists meetups;
drop table if exists users;
