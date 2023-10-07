CREATE TABLE users (
  id uuid PRIMARY KEY,
  username varchar(50) UNIQUE NOT NULL
);

CREATE TABLE iterations (
  id uuid PRIMARY KEY,
  user_id uuid NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  name varchar NOT NULL
);

CREATE TYPE task_status AS ENUM ('active', 'completed');

CREATE TABLE tasks (
  id uuid PRIMARY KEY,
  user_id uuid NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  title varchar NOT NULL,
  status task_status NOT NULL,
  point integer,
  planned_for uuid,
  FOREIGN KEY (planned_for) REFERENCES iterations(id) ON DELETE SET NULL
);
