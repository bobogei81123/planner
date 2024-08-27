-- reverse: drop "iterations" table
CREATE TABLE "public"."iterations" (
  "id" uuid NOT NULL,
  "user_id" uuid NOT NULL,
  "name" character varying NOT NULL,
  PRIMARY KEY ("id"),
  CONSTRAINT "iterations_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "public"."users" ("id") ON UPDATE NO ACTION ON DELETE CASCADE
);
-- reverse: drop "task_schedule" table
CREATE TABLE "public"."task_schedule" (
  "id" uuid NOT NULL,
  "user_id" uuid NOT NULL,
  "date_spec" json NOT NULL,
  "next_date_to_check" date NOT NULL,
  "task_title" character varying NOT NULL,
  "task_point" integer NULL,
  PRIMARY KEY ("id"),
  CONSTRAINT "task_schedule_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "public"."users" ("id") ON UPDATE NO ACTION ON DELETE CASCADE
);
-- reverse: drop enum type "task_status"
CREATE TYPE "public"."task_status" AS ENUM ('active', 'completed');
-- reverse: drop "tasks" table
CREATE TABLE "public"."tasks" (
  "id" uuid NOT NULL,
  "user_id" uuid NOT NULL,
  "title" character varying NOT NULL,
  "status" "public"."task_status" NOT NULL,
  "point" integer NULL,
  "planned_on" date NULL,
  PRIMARY KEY ("id"),
  CONSTRAINT "tasks_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "public"."users" ("id") ON UPDATE NO ACTION ON DELETE CASCADE
);
-- reverse: drop "iterations_tasks" table
CREATE TABLE "public"."iterations_tasks" (
  "iteration_id" uuid NOT NULL,
  "task_id" uuid NOT NULL,
  PRIMARY KEY ("iteration_id", "task_id"),
  CONSTRAINT "iterations_tasks_iteration_id_fkey" FOREIGN KEY ("iteration_id") REFERENCES "public"."iterations" ("id") ON UPDATE NO ACTION ON DELETE CASCADE,
  CONSTRAINT "iterations_tasks_task_id_fkey" FOREIGN KEY ("task_id") REFERENCES "public"."tasks" ("id") ON UPDATE NO ACTION ON DELETE CASCADE
);
CREATE UNIQUE INDEX "iterations_tasks_task_id_key" ON "public"."iterations_tasks" ("task_id");
-- reverse: create "task" table
DROP TABLE "public"."task";
