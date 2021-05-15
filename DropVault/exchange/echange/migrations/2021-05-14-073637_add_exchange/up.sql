-- Your SQL goes here
CREATE TABLE exchanges (
  id SERIAL NOT NULL PRIMARY KEY,
  filename TEXT NOT NULL,  
  usernamedst TEXT NOT NULL,
  usernamesrc TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL
);
