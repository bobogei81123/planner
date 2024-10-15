-- reverse: modify "task" table
ALTER TABLE "public"."task" DROP CONSTRAINT "task_parent_id_fkey", DROP COLUMN "next_recurring_check_date", ADD
 CONSTRAINT "task_user_id_fkey1" FOREIGN KEY ("user_id") REFERENCES "public"."users" ("id") ON UPDATE NO ACTION ON DELETE CASCADE;
