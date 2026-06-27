CREATE TABLE moves (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    image_cover TEXT,
    video_link TEXT,
    notes TEXT,
    category TEXT NOT NULL,
    difficulty INTEGER NOT NULL
);

CREATE TABLE trees (
    id INTEGER PRIMARY KEY NOT NULL,
    system_name TEXT NOT NULL,
    image_cover TEXT,
    root_node_id INTEGER
);

CREATE TABLE tree_nodes (
    id INTEGER PRIMARY KEY NOT NULL,
    tree_id INTEGER NOT NULL,
    move_id INTEGER NOT NULL,
    FOREIGN KEY (tree_id) REFERENCES trees(id),
    FOREIGN KEY (move_id) REFERENCES moves(id)
);

CREATE TABLE node_edges (
    parent_node_id INTEGER NOT NULL,
    child_node_id INTEGER NOT NULL,
    PRIMARY KEY (parent_node_id, child_node_id),
    FOREIGN KEY (parent_node_id) REFERENCES tree_nodes(id),
    FOREIGN KEY (child_node_id) REFERENCES tree_nodes(id)
);

CREATE TABLE todos (
    id INTEGER PRIMARY KEY NOT NULL,
    study_plan_name TEXT NOT NULL
);

CREATE TABLE tasks (
    id INTEGER PRIMARY KEY NOT NULL,
    todo_id INTEGER NOT NULL,
    move_id INTEGER NOT NULL,
    done BOOLEAN NOT NULL DEFAULT 0,
    deadline DATETIME,
    FOREIGN KEY (todo_id) REFERENCES todos(id),
    FOREIGN KEY (move_id) REFERENCES moves(id)
);
