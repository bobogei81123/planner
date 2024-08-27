CREATE TABLE users (
  id uuid PRIMARY KEY,
  username varchar(50) UNIQUE NOT NULL
);

CREATE TABLE task (
  id uuid PRIMARY KEY,
  user_id uuid NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  recurring_spec json,
  scheduled_on json,
  schedule_index_date date,
  complete_date date,
  parent_id uuid,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  title varchar NOT NULL,
  cost integer
);
