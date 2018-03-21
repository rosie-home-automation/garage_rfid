CREATE TABLE logs(
  id TEXT PRIMARY KEY NOT NULL,
  module TEXT NOT NULL,
  action TEXT NOT NULL,
  user_id TEXT NULL,
  data TEXT NULL,
  created_at DATETIME NOT NULL
);
CREATE INDEX idx_logs_user_id on logs(user_id);
CREATE INDEX idx_logs_module on logs(module);
CREATE INDEX idx_logs_created_at on logs(created_at);
