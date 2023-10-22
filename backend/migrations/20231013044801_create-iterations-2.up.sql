-- modify "iterations" table
ALTER TABLE "public"."iterations" ADD COLUMN "start_date" date NULL, ADD COLUMN "end_date" date NULL;
-- modify "tasks" table
ALTER TABLE "public"."tasks" DROP COLUMN "planned_for", ADD COLUMN "planned_on" date NULL;
-- create "iterations_tasks" table
CREATE TABLE "public"."iterations_tasks" (
  "iteration_id" uuid NOT NULL,
  "task_id" uuid NOT NULL,
  PRIMARY KEY ("iteration_id", "task_id"),
  CONSTRAINT "iterations_tasks_iteration_id_fkey" FOREIGN KEY ("iteration_id") REFERENCES "public"."iterations" ("id") ON UPDATE NO ACTION ON DELETE CASCADE,
  CONSTRAINT "iterations_tasks_task_id_fkey" FOREIGN KEY ("task_id") REFERENCES "public"."tasks" ("id") ON UPDATE NO ACTION ON DELETE CASCADE
);
