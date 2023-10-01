CREATE TABLE users (
  id uuid PRIMARY KEY,
  username varchar(50) UNIQUE NOT NULL
);
