-- Your SQL goes here
CREATE TABLE users(
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL UNIQUE,
  nickname VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  password VARCHAR NOT NULL UNIQUE,
  activated BOOLEAN NOT NULL,
  activated_at TIMESTAMP
)
