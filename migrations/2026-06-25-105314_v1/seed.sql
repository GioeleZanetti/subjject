INSERT INTO moves (id, name, image_cover, video_link, notes, category, difficulty) VALUES
(1, 'Closed Guard', NULL, 'https://www.youtube.com/watch?v=Z_FBT8ZDSmo', 'Standard baseline control.', 'guard', 1),
(2, 'Kimura Grip', NULL, NULL, 'Figure-four lock on the arm.', 'grip', 2),
(3, 'Hip Bump Sweep', NULL, NULL, 'Explosive sweep when they sit back.', 'sweep', 2),
(4, 'Kimura Submission', NULL, NULL, 'Breaking mechanics from guard.', 'submission', 3),
(5, 'Guillotine Choke', NULL, NULL, 'Front headlock if they defend the sweep by posting.', 'submission', 3);

INSERT INTO trees (id, system_name, image_cover, root_node_id) VALUES
(1, 'Kimura Trap', NULL, 1);

INSERT INTO tree_nodes (id, tree_id, move_id) VALUES
(1, 1, 1),
(2, 1, 2),
(3, 1, 3),
(4, 1, 4),
(5, 1, 5);

INSERT INTO node_edges (parent_node_id, child_node_id) VALUES
(1, 2),
(2, 3),
(2, 4),
(3, 5);