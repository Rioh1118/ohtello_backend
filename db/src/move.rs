use crate::error::DbError;
use crate::models::NewMove;
use diesel::{insert_into, PgConnection, RunQueryDsl};

pub fn create_move(
    con: &mut PgConnection,
    _game_id: i32,
    _turn_number: i32,
    _player: i32,
    _move_x: Option<i32>,
    _move_y: Option<i32>,
) -> Result<i32, DbError> {
    use crate::schema;
    use crate::schema::moves::dsl::*;

    let new_move = NewMove {
        game_id: _game_id,
        turn_number: _turn_number,
        player: _player,
        move_x: _move_x,
        move_y: _move_y,
        created_at: chrono::Local::now().naive_local(),
    };

    let result = insert_into(moves)
        .values(&new_move)
        .returning(schema::moves::move_id)
        .get_result(con)?;
    Ok(result)
}
