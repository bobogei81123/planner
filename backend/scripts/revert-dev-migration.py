import os
import subprocess
import sys


def get_latest_migration_file_version():
    migrations = (f for f in os.listdir(
        './migrations') if f.endswith('.down.sql'))
    versions = (m.split('_')[0] for m in migrations)

    return max(versions)


def get_latest_sqlx_migration_version():
    last_version_line = subprocess.check_output(
        ['sqlx', 'migrate', 'info'],
        stderr=sys.stderr,
    ).decode().strip().split('\n')[-1]
    return last_version_line.split('/')[0]


def check_call(cmd):
    print(f'Execute {' '.join(cmd)}...')
    subprocess.check_call(
        cmd, stdout=sys.stdout, stderr=sys.stderr)


def remove_migration_file(version):
    for file in os.listdir('./migrations'):
        if file.startswith(version):
            print(f'Removing {file}...')
            os.remove(f'migrations/{file}')


def revert_dev_migration():
    print('First, clean up generated test data')
    check_call(['python', 'scripts/generate-test-data.py', 'clean'])

    while True:
        version = get_latest_migration_file_version()
        if version != (sqlx_version := get_latest_sqlx_migration_version()):
            raise Exception('The versions from migration files and sqlx mismatch: '
                            f'file={version}, sqlx={sqlx_version}')

        print(f'Revert {version}? [y/n]:', end=' ')
        if input() != 'y':
            break

        print(f'Reverting migration {version}')
        check_call(['sqlx', 'migrate', 'revert'])

        print(f'Remove migration file {version}')
        remove_migration_file(version)

    check_call(['npx', '@ariga/atlas', 'migrate', 'hash'])


def main():
    revert_dev_migration()


if __name__ == '__main__':
    main()
