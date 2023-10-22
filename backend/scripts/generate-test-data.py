#!/usr/bin/env python

import enum
import dataclasses
import os
import sys
import uuid

import dotenv
import psycopg2
import psycopg2.extensions
import psycopg2.extras
from psycopg2.extras import DateRange

psycopg2.extras.register_uuid()


def get_db_conn():
    return psycopg2.connect(os.getenv('DATABASE_URL'))


@dataclasses.dataclass
class User:
    id: uuid.UUID
    username: str


@dataclasses.dataclass
class Iteration:
    id: uuid.UUID
    user_id: uuid.UUID
    name: str
    date_range: DateRange | None


@enum.unique
class TaskStatus(enum.Enum):
    ACTIVE = 'active'
    COMPLETED = 'completed'

    def __conform__(self, protocol):
        if protocol is psycopg2.extensions.ISQLQuote:
            return psycopg2.extensions.adapt(self.value, protocol)


@dataclasses.dataclass
class Tasks:
    id: uuid.UUID
    user_id: uuid.UUID
    title: str
    status: TaskStatus
    point: int | None


METEOR_UUID = uuid.UUID('00000000-0000-4000-8001-000000000000')


def task_uuid(n: int):
    return uuid.UUID(f'00000000-0000-4000-8002-0000000000{n:02}')


def iteration_uuid(n: int):
    return uuid.UUID(f'00000000-0000-4000-8003-0000000000{n:02}')


def generate_test_data(db_conn):
    meteor = User(METEOR_UUID, 'meteor')
    users = [meteor, User(uuid.uuid4(), 'test')]
    iterations = [
        Iteration(iteration_uuid(1), meteor.id, 'Iteration #1',
                  DateRange('2023-10-01', '2024-10-03')),
        Iteration(iteration_uuid(2), meteor.id, 'Iteration #2',
                  DateRange('2024-10-04', '2025-10-03')),
    ]
    tasks = [
        Tasks(task_uuid(1), meteor.id, 'Task #1', TaskStatus.ACTIVE, 1),
        Tasks(task_uuid(2), meteor.id, 'Task #2', TaskStatus.COMPLETED, None),
        Tasks(task_uuid(3), meteor.id, 'Task #3', TaskStatus.ACTIVE, 3),
        Tasks(task_uuid(4), meteor.id, 'Task #4', TaskStatus.COMPLETED, 4),
    ]
    iterations_task = [
        (iterations[0].id, tasks[0].id),
        (iterations[0].id, tasks[1].id),
        (iterations[0].id, tasks[2].id),
    ]

    with db_conn.cursor() as cursor:
        for user in users:
            cursor.execute(
                'INSERT INTO users (id, username) VALUES (%s, %s)',
                (user.id, user.username)
            )
        for task in tasks:
            cursor.execute(
                'INSERT INTO tasks (id, user_id, title, status, point) '
                'VALUES (%s, %s, %s, %s, %s)',
                (task.id, task.user_id, task.title, task.status, task.point)
            )
        for iteration in iterations:
            cursor.execute(
                'INSERT INTO iterations (id, user_id, name, date_range) '
                'VALUES (%s, %s, %s, %s)',
                (iteration.id, iteration.user_id, iteration.name,
                 iteration.date_range)
            )
        for (iteration_id, task_id) in iterations_task:
            cursor.execute(
                'INSERT INTO iterations_tasks (iteration_id, task_id) '
                'VALUES (%s, %s)',
                (iteration_id, task_id)
            )
        db_conn.commit()


def clean_db(db_conn):
    with db_conn.cursor() as cursor:
        tables = ['users', 'tasks', 'iterations', 'iterations_tasks']
        for table in tables:
            cursor.execute(f'DELETE FROM {table};')
        db_conn.commit()


if __name__ == '__main__':
    if len(sys.argv) != 2 or sys.argv[1] not in ('generate', 'clean'):
        print(f'Usage: {sys.argv[0]} <generate|clean>')
        sys.exit(1)

    dotenv.load_dotenv()
    with get_db_conn() as db_conn:
        match sys.argv[1]:
            case 'generate':
                generate_test_data(db_conn)
            case 'clean':
                clean_db(db_conn)
