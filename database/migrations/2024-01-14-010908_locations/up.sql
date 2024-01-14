-- Your SQL goes here
create table locations (
    id serial not null primary key,
    address varchar(40) not null,
    city varchar(40) not null,
    lat numeric not null,
    lng numeric not null
);