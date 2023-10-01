CREATE TYPE task_status AS ENUM ('active', 'completed');

CREATE TABLE tasks (
  id uuid PRIMARY KEY,
  user_id uuid NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  title varchar NOT NULL,
  status task_status NOT NULL,
  point integer
);
