DO $$
DECLARE
  meteor_id uuid := '00000000-0000-0000-0001-000000000000';
BEGIN
  INSERT INTO users(id, username) VALUES
    ('00000000-0000-0000-0001-000000000000', 'meteor'),
    ('00000000-0000-0000-0001-000000000001', 'haha');

  INSERT INTO tasks(id, user_id, title, status, point) VALUES
    ('00000000-0000-0000-0002-000000000000', meteor_id, 'Meteor Task #1', 'active', 3),
    ('00000000-0000-0000-0002-000000000001', meteor_id, 'Meteor Task #2', 'completed', 1),
    ('00000000-0000-0000-0002-000000000002', meteor_id, 'Meteor Task #3', 'active', NULL);
END $$
