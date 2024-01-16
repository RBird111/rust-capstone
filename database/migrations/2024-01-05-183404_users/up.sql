CREATE TABLE users (
  id serial NOT NULL PRIMARY KEY,
  first_name varchar(40) NOT NULL,
  last_name varchar(40) NOT NULL,
  username varchar(40) NOT NULL UNIQUE,
  email varchar(40) NOT NULL UNIQUE,
  hashed_password text NOT NULL
);