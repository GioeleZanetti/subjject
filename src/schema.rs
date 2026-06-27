// @generated automatically by Diesel CLI.

diesel::table! {
    moves (id) {
        id -> Integer,
        name -> Text,
        image_cover -> Nullable<Text>,
        video_link -> Nullable<Text>,
        notes -> Nullable<Text>,
        category -> Text,
        difficulty -> Integer,
    }
}

diesel::table! {
    node_edges (parent_node_id, child_node_id) {
        parent_node_id -> Integer,
        child_node_id -> Integer,
    }
}

diesel::table! {
    tasks (id) {
        id -> Integer,
        todo_id -> Integer,
        move_id -> Integer,
        done -> Bool,
        deadline -> Nullable<Timestamp>,
    }
}

diesel::table! {
    todos (id) {
        id -> Integer,
        study_plan_name -> Text,
    }
}

diesel::table! {
    tree_nodes (id) {
        id -> Integer,
        tree_id -> Integer,
        move_id -> Integer,
    }
}

diesel::table! {
    trees (id) {
        id -> Integer,
        system_name -> Text,
        image_cover -> Nullable<Text>,
        root_node_id -> Nullable<Integer>,
    }
}

diesel::joinable!(tasks -> moves (move_id));
diesel::joinable!(tasks -> todos (todo_id));
diesel::joinable!(tree_nodes -> moves (move_id));
diesel::joinable!(tree_nodes -> trees (tree_id));

diesel::allow_tables_to_appear_in_same_query!(
    moves,
    node_edges,
    tasks,
    todos,
    tree_nodes,
    trees,
);
