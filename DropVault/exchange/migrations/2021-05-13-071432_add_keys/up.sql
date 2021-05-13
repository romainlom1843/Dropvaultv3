-- Your SQL goes here
CREATE TABLE pubkeys (
  id SERIAL NOT NULL PRIMARY KEY,
  pubkey TEXT NOT NULL,
  username TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL
);
/*CREATE TABLE exchanges (
  id SERIAL NOT NULL PRIMARY KEY,
  filename TEXT NOT NULL,
  fileid SERIAL NOT NULL,
  username TEXT NOT NULL,
  login Text NOT NULL,
  created_at TIMESTAMP NOT NULL
);*/
