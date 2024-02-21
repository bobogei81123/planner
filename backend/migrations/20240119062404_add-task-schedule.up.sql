-- create "task_schedule" table
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
