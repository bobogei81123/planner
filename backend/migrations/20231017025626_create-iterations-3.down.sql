-- reverse: create index "iterations_tasks_task_id_key" to table: "iterations_tasks"
DROP INDEX "public"."iterations_tasks_task_id_key";
-- reverse: modify "iterations" table
ALTER TABLE "public"."iterations" DROP COLUMN "date_range", ADD COLUMN "end_date" date NULL, ADD COLUMN "start_date" date NULL;
