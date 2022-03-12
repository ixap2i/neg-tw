-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  user_type VARCHAR NOT NULL,
  udid TEXT NOT NULL
)