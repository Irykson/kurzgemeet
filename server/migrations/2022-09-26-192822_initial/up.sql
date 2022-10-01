create table users
(
    id   serial primary key,
    name varchar not null
);

create table password_users
(
    id       serial primary key,
    password varchar not null,
    user_id  integer not null references users
);

create table email_users
(
    id      serial primary key,
    email   varchar not null,
    user_id integer not null references users
);

create table meetups
(
    id          serial primary key,
    password    varchar not null,
    name        varchar not null,
    description varchar not null,
    user_id     integer not null references users
);

create table meetup_dates
(
    id        serial primary key,
    date      date    not null,
    meetup_id integer not null references meetups
);

create table meetup_places
(
    id        serial primary key,
    place     varchar not null,
    meetup_id integer not null references meetups
);

create table meetup_date_votes
(
    id             serial primary key,
    meetup_date_id integer not null references meetup_dates,
    user_id        integer not null references users
);

create table meetup_place_votes
(
    id              serial primary key,
    meetup_place_id integer not null references meetup_places,
    user_id         integer not null references users
);

-- due to cross reference, this must be added later
alter table meetups
    add column meetup_date_id integer not null references meetup_dates;
alter table meetups
    add column meetup_place_id integer not null references meetup_places;

