-- reverse: modify "iterations" table
ALTER TABLE "public"."iterations" ADD COLUMN "date_range" daterange NULL;
