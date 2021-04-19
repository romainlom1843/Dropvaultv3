-- Your SQL goes here
CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  username TEXT NOT NULL,
  email TEXT NOT NULL,
  hashe TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL
);

