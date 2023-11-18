-- create "iterations" table
CREATE TABLE "public"."iterations" (
  "id" uuid NOT NULL,
  "user_id" uuid NOT NULL,
  "name" character varying NOT NULL,
  "date_range" daterange NULL,
  PRIMARY KEY ("id"),
  CONSTRAINT "iterations_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "public"."users" ("id") ON UPDATE NO ACTION ON DELETE CASCADE
);
-- modify "tasks" table
ALTER TABLE "public"."tasks" ADD COLUMN "planned_on" date NULL;
-- create "iterations_tasks" table
CREATE TABLE "public"."iterations_tasks" (
  "iteration_id" uuid NOT NULL,
  "task_id" uuid NOT NULL,
  PRIMARY KEY ("iteration_id", "task_id"),
  CONSTRAINT "iterations_tasks_iteration_id_fkey" FOREIGN KEY ("iteration_id") REFERENCES "public"."iterations" ("id") ON UPDATE NO ACTION ON DELETE CASCADE,
  CONSTRAINT "iterations_tasks_task_id_fkey" FOREIGN KEY ("task_id") REFERENCES "public"."tasks" ("id") ON UPDATE NO ACTION ON DELETE CASCADE
);
-- create index "iterations_tasks_task_id_key" to table: "iterations_tasks"
CREATE UNIQUE INDEX "iterations_tasks_task_id_key" ON "public"."iterations_tasks" ("task_id");
