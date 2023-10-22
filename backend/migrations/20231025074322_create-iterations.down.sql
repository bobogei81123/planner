-- reverse: create index "iterations_tasks_task_id_key" to table: "iterations_tasks"
DROP INDEX "public"."iterations_tasks_task_id_key";
-- reverse: create "iterations_tasks" table
DROP TABLE "public"."iterations_tasks";
-- reverse: modify "tasks" table
ALTER TABLE "public"."tasks" DROP COLUMN "planned_on";
-- reverse: create "iterations" table
DROP TABLE "public"."iterations";
