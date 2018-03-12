CREATE TABLE credentials(
  id VARCHAR PRIMARY KEY NOT NULL,
  user_id VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  variety VARCHAR NOT NULL,
  value VARCHAR NOT NULL,
  created_at DATETIME NOT NULL,
  updated_at DATETIME NOT NULL
);
CREATE INDEX idx_credentials_user_id ON credentials(user_id);
CREATE UNIQUE INDEX udx_credentials_variety_value ON credentials(variety, value);
