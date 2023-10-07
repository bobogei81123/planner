DO $$
DECLARE
  meteor_id uuid := '00000000-0000-4000-8001-000000000000';
  iter_1_id uuid := '00000000-0000-4000-8003-000000000000';
  iter_2_id uuid := '00000000-0000-4000-8003-000000000001';
BEGIN
  INSERT INTO users(id, username) VALUES
    (meteor_id, 'meteor'),
    ('00000000-0000-4000-8001-000000000001', 'haha');

  INSERT INTO tasks(id, user_id, title, status, point) VALUES
    ('00000000-0000-4000-8002-000000000000', meteor_id, 'Meteor Task: Unplanned, Active', 'active', 3),
    ('00000000-0000-4000-8002-000000000001', meteor_id, 'Meteor Task: Unplanned, Completed', 'completed', 1),
    ('00000000-0000-4000-8002-000000000002', meteor_id, 'Meteor Task: Unplanned, Point is NULL', 'active', NULL);

  INSERT INTO iterations(id, user_id, name) VALUES
    (iter_1_id, meteor_id, 'Meteor Iteration #1'),
    (iter_2_id, meteor_id, 'Meteor Iteration #2');

  INSERT INTO tasks(id, user_id, title, status, point, planned_for) VALUES
    ('00000000-0000-4000-8002-000000000003', meteor_id, 'Meteor Task: Iter #1, Active', 'active', 5, iter_1_id),
    ('00000000-0000-4000-8002-000000000004', meteor_id, 'Meteor Task: Iter #1, Completed', 'completed', 1, iter_1_id),
    ('00000000-0000-4000-8002-000000000005', meteor_id, 'Meteor Task: Iter #2, Active', 'active', NULL, iter_2_id);
END $$
