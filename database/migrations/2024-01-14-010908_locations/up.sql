CREATE TABLE locations (
    id serial NOT NULL PRIMARY KEY,
    address varchar(40) NOT NULL,
    city varchar(40) NOT NULL,
    state varchar(40) NOT NULL,
    lat numeric(7, 4),
    lng numeric(7, 4)
);