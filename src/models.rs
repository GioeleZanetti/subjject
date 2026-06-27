use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::sqlite::SqliteConnection>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Submission,
    Escape,
    Guard,
    Grip,
    Takedown,
    Pass,
    Sweep,
}

impl From<String> for Category {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "submission" => Category::Submission,
            "escape" => Category::Escape,
            "guard" => Category::Guard,
            "grip" => Category::Grip,
            "takedown" => Category::Takedown,
            "pass" => Category::Pass,
            "sweep" => Category::Sweep,
            _ => Category::Guard, 
        }
    }
}

impl Into<String> for Category {
    fn into(self) -> String {
        match self {
            Category::Submission => String::from("submission"),
            Category::Escape => String::from("escape"),
            Category::Guard => String::from("guard"),
            Category::Grip => String::from("grip"),
            Category::Takedown => String::from("takedown"),
            Category::Pass => String::from("pass"),
            Category::Sweep => String::from("sweep"),
        }
    }
}

impl ToString for Category {
    
    fn to_string(&self) -> String {
        match self {
            Category::Submission => String::from("submission"),
            Category::Escape => String::from("escape"),
            Category::Guard => String::from("guard"),
            Category::Grip => String::from("grip"),
            Category::Takedown => String::from("takedown"),
            Category::Pass => String::from("pass"),
            Category::Sweep => String::from("sweep"),
        }
    }
}

impl Category {
    pub const ALL: [Self; 7] = [
        Self::Submission,
        Self::Escape,
        Self::Guard,
        Self::Grip,
        Self::Takedown,
        Self::Pass,
        Self::Sweep,
    ];
}

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::moves)]
pub struct Move {
    pub id: i32,
    pub name: String,
    pub image_cover: Option<String>,
    pub video_link: Option<String>,
    pub notes: Option<String>,
    pub category: String,
    pub difficulty: i32,
}

#[derive(Insertable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = crate::schema::moves)]
pub struct NewMove {
    pub name: String,
    pub image_cover: Option<String>,
    pub video_link: Option<String>,
    pub notes: Option<String>,
    pub category: String,
    pub difficulty: i32,
}

#[derive(AsChangeset, Deserialize, Debug)]
#[diesel(table_name = crate::schema::moves)]
pub struct UpdateMove {
    pub name: Option<String>,
    pub image_cover: Option<String>,
    pub video_link: Option<String>,
    pub notes: Option<String>,
    pub category: Option<String>,
    pub difficulty: Option<i32>,
}

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::trees)]
pub struct Tree {
    pub id: i32,
    pub system_name: String,
    pub image_cover: Option<String>,
    pub root_node_id: Option<i32>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::trees)]
pub struct NewTree {
    pub system_name: String,
    pub image_cover: Option<String>,
    pub root_node_id: Option<i32>,
}

#[derive(AsChangeset, Deserialize, Debug)]
#[diesel(table_name = crate::schema::trees)]
pub struct UpdateTree {
    pub system_name: Option<String>,
    pub image_cover: Option<String>,
    pub root_node_id: Option<i32>,
}

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::tree_nodes)]
pub struct TreeNode {
    pub id: i32,
    pub tree_id: i32,
    pub move_id: i32,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::tree_nodes)]
pub struct NewTreeNode {
    pub tree_id: i32,
    pub move_id: i32,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::node_edges)]
#[diesel(primary_key(parent_node_id, child_node_id))]
pub struct _NodeEdge {
    pub parent_node_id: i32,
    pub child_node_id: i32,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::node_edges)]
pub struct NewNodeEdge {
    pub parent_node_id: i32,
    pub child_node_id: i32,
}

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::todos)]
pub struct Todo {
    pub id: i32,
    pub study_plan_name: String,
}

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::tasks)]
pub struct Task {
    pub id: i32,
    pub todo_id: i32,
    pub move_id: i32,
    pub done: bool,
    pub deadline: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct HomeStats {
    pub total: i64,
    pub submissions: i64,
    pub takedowns: i64,
    pub guards: i64,
    pub passes: i64,
    pub escapes: i64,
}