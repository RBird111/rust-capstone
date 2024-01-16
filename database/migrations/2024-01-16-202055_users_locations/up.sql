CREATE TABLE users_locations (
    user_id integer REFERENCES users (id),
    location_id integer REFERENCES locations (id),
    PRIMARY KEY(user_id, location_id)
);