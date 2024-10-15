-- modify "task" table
ALTER TABLE "public"."task" DROP CONSTRAINT "task_user_id_fkey1", ADD COLUMN "next_recurring_check_date" date NULL, ADD
 CONSTRAINT "task_parent_id_fkey" FOREIGN KEY ("parent_id") REFERENCES "public"."task" ("id") ON UPDATE NO ACTION ON DELETE SET NULL;
