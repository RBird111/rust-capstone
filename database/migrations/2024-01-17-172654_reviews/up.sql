CREATE TABLE reviews (
    id serial NOT NULL PRIMARY KEY,
    rating integer NOT NULL,
    body text NOT NULL,
    user_id integer NOT NULL REFERENCES users (id),
    business_id integer NOT NULL REFERENCES businesses (id)
);