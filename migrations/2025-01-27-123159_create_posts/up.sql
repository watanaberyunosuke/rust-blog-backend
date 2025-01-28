-- Your SQL goes here
CREATE TABLE posts (
                       id SERIAL PRIMARY KEY,
                       title VARCHAR NOT NULL,
                       body TEXT NOT NULL,
                       post_type VARCHAR NOT NULL,
                       published BOOLEAN NOT NULL DEFAULT false
)
