-- modify "iterations" table
ALTER TABLE "public"."iterations" DROP COLUMN "start_date", DROP COLUMN "end_date", ADD COLUMN "date_range" daterange NULL;
-- create index "iterations_tasks_task_id_key" to table: "iterations_tasks"
CREATE UNIQUE INDEX "iterations_tasks_task_id_key" ON "public"."iterations_tasks" ("task_id");
