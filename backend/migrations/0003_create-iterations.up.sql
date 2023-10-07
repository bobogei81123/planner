BEGIN;

CREATE TABLE iterations (
  id uuid PRIMARY KEY,
  user_id uuid NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  name varchar NOT NULL
);

ALTER TABLE tasks
ADD COLUMN planned_for uuid,
ADD CONSTRAINT tasks_planned_for_fkey
  FOREIGN KEY (planned_for) REFERENCES iterations(id) ON DELETE SET NULL;

COMMIT;
