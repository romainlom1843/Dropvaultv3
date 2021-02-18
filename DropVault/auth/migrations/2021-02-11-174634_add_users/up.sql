-- Your SQL goes here
CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  username TEXT NOT NULL,
  email TEXT NOT NULL,
  hashe TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL
);
CREATE TABLE tokens (
  token_id SERIAL NOT NULL PRIMARY KEY,
  login TEXT NOT NULL,
  token TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL
);
