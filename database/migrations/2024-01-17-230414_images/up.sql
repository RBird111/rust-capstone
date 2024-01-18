CREATE TABLE images (
    id serial NOT NULL PRIMARY KEY,
    url text NOT NULL,
    user_id integer NOT NULL REFERENCES users (id),
    business_id integer REFERENCES businesses (id),
    review_id integer REFERENCES reviews (id)
);