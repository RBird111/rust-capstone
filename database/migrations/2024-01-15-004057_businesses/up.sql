CREATE TABLE businesses (
    id serial NOT NULL PRIMARY KEY,
    name varchar(40) NOT NULL,
    description text NOT NULL,
    category varchar(40) NOT NULL,
    location_id integer NOT NULL REFERENCES locations (id),
    owner_id integer REFERENCES users (id)
);