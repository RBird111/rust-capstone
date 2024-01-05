create table users (
  id serial not null primary key,
  first_name varchar(40) not null,
  last_name varchar(40) not null,
  username varchar(40) not null unique,
  email varchar(40) not null unique,
  hashed_password text not null
);