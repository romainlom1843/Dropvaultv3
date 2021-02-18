-- Your SQL goes here
CREATE TABLE files (
  id SERIAL NOT NULL PRIMARY KEY,
  filename TEXT NOT NULL,
  content TEXT NOT NULL,
  username TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL
);
