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
  planned_on date
);

CREATE TABLE iterations_tasks (
  iteration_id uuid NOT NULL,
  FOREIGN KEY (iteration_id) REFERENCES iterations(id) ON DELETE CASCADE,
  task_id uuid UNIQUE NOT NULL,
  FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
  PRIMARY KEY (iteration_id, task_id)
);
