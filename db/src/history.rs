use crate::error::DbError;
use crate::models::*;
use diesel::prelude::*;
use diesel::{insert_into, pg::PgConnection};

pub fn create_history(
    con: &mut PgConnection,
    _game_id: i32,
    _move_id: i32,
) -> Result<i32, DbError> {
    use crate::schema;
    use crate::schema::histories::dsl::*;

    let new_history = NewHistory {
        game_id: _game_id,
        move_id: _move_id,
        created_at: chrono::Local::now().naive_local(),
    };
    let result = insert_into(histories)
        .values(&new_history)
        .returning(schema::histories::history_id)
        .get_result(con)?;
    Ok(result)
}

pub fn get_history_by_game_id(
    con: &mut PgConnection,
    _game_id: i32,
) -> Result<Vec<History>, DbError> {
    use crate::schema::histories::dsl::*;

    let history_vec = histories.filter(game_id.eq(_game_id)).load(con)?;

    Ok(history_vec)
}
