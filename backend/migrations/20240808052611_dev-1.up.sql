-- create "task" table
CREATE TABLE "public"."task" (
  "id" uuid NOT NULL,
  "user_id" uuid NOT NULL,
  "recurring_spec" json NULL,
  "scheduled_on" json NULL,
  "schedule_index_date" date NULL,
  "complete_date" date NULL,
  "parent_id" uuid NULL,
  "title" character varying NOT NULL,
  "cost" integer NULL,
  PRIMARY KEY ("id"),
  CONSTRAINT "task_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "public"."users" ("id") ON UPDATE NO ACTION ON DELETE CASCADE,
  CONSTRAINT "task_user_id_fkey1" FOREIGN KEY ("user_id") REFERENCES "public"."users" ("id") ON UPDATE NO ACTION ON DELETE CASCADE
);
-- drop "iterations_tasks" table
DROP TABLE "public"."iterations_tasks";
-- drop "tasks" table
DROP TABLE "public"."tasks";
-- drop enum type "task_status"
DROP TYPE "public"."task_status";
-- drop "task_schedule" table
DROP TABLE "public"."task_schedule";
-- drop "iterations" table
DROP TABLE "public"."iterations";
