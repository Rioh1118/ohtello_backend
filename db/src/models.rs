use diesel::prelude::*;
use crate::schema::*;
#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = games)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Game {
    pub game_id: i32,
    pub player1_id: i32,
    pub player2_id: i32,
    pub current_turn: i32,
    pub status: String,
    pub black_board: String,
    pub white_board: String,
    pub created_at: chrono::NaiveDateTime,
    pub finished_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = games)]
pub struct NewGame<'a> {
    pub player1_id: i32,
    pub player2_id: i32,
    pub current_turn: i32,
    pub status: &'a str,
    pub black_board: &'a str,
    pub white_board: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub finished_at: Option<chrono::NaiveDateTime>,
}
#[derive(Queryable, Selectable)]
#[diesel(table_name = moves)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Move {
    pub move_id: i32,
    pub game_id: i32,
    pub turn_number: i32,
    pub player: i32, // 1or2
    pub move_x: Option<i32>,
    pub move_y: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = moves)]
pub struct NewMove {
    pub game_id: i32,
    pub turn_number: i32,
    pub player: i32, //1or2
    pub move_x: Option<i32>,
    pub move_y: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = histories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct History {
    pub history_id: i32,
    pub game_id: i32,
    pub move_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = histories)]
pub struct NewHistory {
    pub game_id: i32,
    pub move_id: i32,
    pub created_at: chrono::NaiveDateTime,
}