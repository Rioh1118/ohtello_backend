use crate::error::DbError;
use crate::models::*;
use diesel::prelude::*;
use diesel::{insert_into, pg::PgConnection, update};

pub fn create_new_game(
    con: &mut PgConnection,
    player1_id: i32,
    player2_id: i32,
    status: &str,
) -> Result<i32, DbError> {
    use crate::schema;
    use crate::schema::games::dsl::games;

    let new_game = NewGame {
        player1_id,
        player2_id,
        status,
        current_turn: 1,
        black_board: "0000000000000000000000000000100000010000000000000000000000000000",
        white_board: "0000000000000000000000000001000000001000000000000000000000000000",
        created_at: chrono::Local::now().naive_local(),
        finished_at: None,
    };

    let result = insert_into(games)
        .values(&new_game)
        .returning(schema::games::game_id)
        .get_result(con)?;
    Ok(result)
}

pub fn get_game_by_id(con: &mut PgConnection, id: i32) -> Result<Game, DbError> {
    use crate::schema::games::dsl::*;

    let game = games.filter(game_id.eq(id)).first(con)?;

    Ok(game)
}

pub fn get_games_by_player_id(
    con: &mut PgConnection,
    player_id: i32,
) -> Result<Vec<Game>, DbError> {
    use crate::schema::games::dsl::*;

    let games_vec = games
        .filter(player1_id.eq(player_id).or(player2_id.eq(player_id)))
        .load(con)?;

    Ok(games_vec)
}

pub fn update_game_phase<'a>(
    con: &mut PgConnection,
    _game_id: i32,
    black: &'a str,
    white: &'a str,
    next_turn: i32,
    _status: &'a str,
) -> Result<(), DbError> {
    use crate::schema::games::dsl::*;

    update(games.filter(game_id.eq(_game_id)))
        .set((
            black_board.eq(black),
            white_board.eq(white),
            current_turn.eq(next_turn),
            status.eq(_status),
        ))
        .execute(con)?;

    Ok(())
}
