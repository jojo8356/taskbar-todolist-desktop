CREATE TABLE IF NOT EXISTS tasks (
  id TEXT PRIMARY KEY NOT NULL,
  text TEXT NOT NULL CHECK (length(trim(text)) > 0),
  status TEXT NOT NULL CHECK (status IN ('todo', 'done')),
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_tasks_active_created_at
ON tasks (deleted_at, created_at DESC);
