create table businesses (
    id serial not null primary key,
    name varchar(40) not null,
    description text not null,
    category varchar(40) not null,
    location_id integer not null references locations (id),
    owner_id integer references users (id)
);