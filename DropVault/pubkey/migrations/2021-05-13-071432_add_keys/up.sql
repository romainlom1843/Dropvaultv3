-- Your SQL goes here
CREATE TABLE pubkeys (
  id SERIAL NOT NULL PRIMARY KEY,
  pubkey TEXT NOT NULL,
  username TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL
);

