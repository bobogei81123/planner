#!/usr/bin/env python

import enum
import dataclasses
import os
import sys
import uuid
import datetime

import dotenv
import psycopg2
import psycopg2.extensions
import psycopg2.extras
from psycopg2.extras import DateRange, Json

psycopg2.extras.register_uuid()


def get_db_conn():
    return psycopg2.connect(os.getenv("DATABASE_URL"))


ALL_TABLES: list[str] = []
DATA_CLASS_TABLE_MAP: dict[type, str] = {}


def sql_table(table_name=None):
    def decorator(cls):
        _table_name = table_name
        if _table_name is None:
            _table_name = cls.__name__.lower()
        ALL_TABLES.append(_table_name)
        DATA_CLASS_TABLE_MAP[cls] = _table_name
        return cls

    return decorator


@sql_table("users")
@dataclasses.dataclass
class User:
    id: uuid.UUID
    username: str


@sql_table("task")
@dataclasses.dataclass
class Task:
    id: uuid.UUID
    user_id: uuid.UUID
    scheduled_on: Json | None
    title: str
    cost: int | None


METEOR_UUID = uuid.UUID("00000000-0000-4000-8001-000000000000")


def task_uuid(n: int):
    return uuid.UUID(f"00000000-0000-4000-8002-0000000000{n:02}")


def generate_test_data():
    meteor = User(METEOR_UUID, "meteor")
    users = [meteor, User(uuid.uuid4(), "test")]
    tasks = [
        Task(
            task_uuid(1),
            meteor.id,
            Json({"Date": "2024-08-03"}),
            "Task #1",
            1,
        ),
        Task(
            task_uuid(2),
            meteor.id,
            Json({"Week": {"start_date": "2024-08-05"}}),
            "Task #2",
            3,
        ),
        Task(task_uuid(3), meteor.id, None, "Task #3", None),
    ]

    return [*users, *tasks]


def run_sql(cursor, *args):
    try:
        cursor.execute(*args)
    except Exception as e:
        if cursor.query is not None:
            print(f"Failed to execute `{cursor.query.decode()}`")
        else:
            print(f"Failed to format sql with args={args}")
        raise e
    print(f"Executed `{cursor.query.decode()}`")


def insert_test_data(db_conn):
    test_data = generate_test_data()

    with db_conn.cursor() as cursor:
        for data in test_data:
            table = DATA_CLASS_TABLE_MAP[type(data)]
            datadict = dataclasses.asdict(data)
            fields = "(" + ", ".join(datadict.keys()) + ")"
            value_placeholders = "(" + ", ".join(["%s"] * len(datadict)) + ")"
            sql = f"INSERT INTO {table} {fields} VALUES {value_placeholders};"
            run_sql(cursor, sql, tuple(datadict.values()))
        db_conn.commit()


def clean_db(db_conn):
    with db_conn.cursor() as cursor:
        for table in ALL_TABLES:
            run_sql(cursor, f"DELETE FROM {table};")
        db_conn.commit()


if __name__ == "__main__":
    if len(sys.argv) != 2 or sys.argv[1] not in ("generate", "clean"):
        print(f"Usage: {sys.argv[0]} <generate|clean>")
        sys.exit(1)

    dotenv.load_dotenv()
    with get_db_conn() as db_conn:
        match sys.argv[1]:
            case "generate":
                insert_test_data(db_conn)
            case "clean":
                clean_db(db_conn)
