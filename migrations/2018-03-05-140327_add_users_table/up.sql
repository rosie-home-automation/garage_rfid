CREATE TABLE users (
  id VARCHAR PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL,
  created_at DATETIME NOT NULL,
  updated_at DATETIME NOT NULL
);
CREATE UNIQUE INDEX udx_users_name ON users(name);
