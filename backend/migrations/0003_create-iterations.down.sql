BEGIN;

ALTER TABLE tasks
DROP CONSTRAINT tasks_planned_for_fkey,
DROP COLUMN planned_for;

DROP TABLE iterations;

COMMIT;
