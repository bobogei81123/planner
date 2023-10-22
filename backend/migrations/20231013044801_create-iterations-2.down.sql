-- reverse: create "iterations_tasks" table
DROP TABLE "public"."iterations_tasks";
-- reverse: modify "tasks" table
ALTER TABLE "public"."tasks" DROP COLUMN "planned_on", ADD COLUMN "planned_for" uuid NULL;
-- reverse: modify "iterations" table
ALTER TABLE "public"."iterations" DROP COLUMN "end_date", DROP COLUMN "start_date";
