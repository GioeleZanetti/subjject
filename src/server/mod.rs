use leptos::prelude::*;
use crate::{models::{Category, HomeStats}, schema::moves::dsl::*};
use diesel::prelude::*;

use crate::models::{DbPool, Move, NewMove};

#[cfg(feature = "ssr")]
pub async fn get_connection() -> Result<
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::sqlite::SqliteConnection>>, 
    ServerFnError
> {
    use actix_web::web;
    use leptos_actix::extract;

    let pool = match extract::<web::Data<DbPool>>().await {
        Ok(p) => p,
        Err(_) => return Err(ServerFnError::ServerError("Database pool missing".into())),
    };

    match pool.get() {
        Ok(conn) => Ok(conn),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(GetMoves, "/api")]
pub async fn get_moves() -> Result<Vec<Move>, ServerFnError> {
    let mut conn = get_connection().await?;
    
    let all_moves = match moves.load::<Move>(&mut conn) {
        Ok(m) => m,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };

    Ok(all_moves)
}

#[server(GetMoveById, "/api")]
pub async fn get_move_by_id(move_id: i32) -> Result<Option<Move>, ServerFnError> {
    let mut conn = get_connection().await?;
    
    let single_move = match moves.find(move_id).first::<Move>(&mut conn).optional() {
        Ok(m) => m,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };

    Ok(single_move)
}

#[server(AddMove, "/api")]
pub async fn add_move(
    new_name: String, 
    new_category: String, 
    new_difficulty: i32, 
    new_video_link: Option<String>,
    new_notes: Option<String>,
    new_image_cover: Option<String>
) -> Result<Move, ServerFnError> {
    use crate::schema::moves::dsl::*;
    use diesel::prelude::*;

    let mut conn = get_connection().await?;

    let new_move = NewMove {
        name: new_name,
        category: new_category,
        difficulty: new_difficulty,
        video_link: new_video_link.filter(|s| !s.is_empty()),
        notes: new_notes.filter(|s| !s.is_empty()),
        image_cover: new_image_cover.filter(|s| !s.is_empty()),
    };
    
    let result = diesel::insert_into(moves)
        .values(&new_move)
        .returning(Move::as_returning())
        .get_result(&mut conn);

    match result {
        Ok(inserted_move) => Ok(inserted_move),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(DeleteMoveById, "/api")]
pub async fn delete_move_by_id(move_id: i32) -> Result<(), ServerFnError> {
    use crate::schema::moves::dsl::*;
    use diesel::prelude::*;

    let mut conn = get_connection().await?;
    
    let result = diesel::delete(moves.find(move_id))
        .execute(&mut conn);

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(GetHomeStats, "/api")]
pub async fn get_home_stats() -> Result<HomeStats, ServerFnError> {
    use crate::schema::moves::dsl::*;
    use diesel::prelude::*;

    let mut conn = get_connection().await?;

    let total_count: i64 = moves.count().get_result(&mut conn).unwrap_or(0);
    
    let sub_count: i64 = moves.filter(category.eq(Category::Submission.to_string()))
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);
        
    let td_count: i64 = moves.filter(category.eq(Category::Takedown.to_string()))
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);

    let guard_count: i64 = moves.filter(category.eq(Category::Guard.to_string()))
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);

    let pass_count: i64 = moves.filter(category.eq(Category::Pass.to_string()))
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);

    let escape_count: i64 = moves.filter(category.eq(Category::Escape.to_string()))
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);

    Ok(HomeStats {
        total: total_count,
        submissions: sub_count,
        takedowns: td_count,
        guards: guard_count,
        passes: pass_count,
        escapes: escape_count
    })
}


