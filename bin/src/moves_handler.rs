use actix_web::{get, post, web, HttpResponse, Responder};
use db::error::DbError;
use db::models::User;
use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct  NewMove {
    pub move_id: i32,
    pub game_id: i32,
    pub turn_number: i32,
    pub player: i32, // 1or2
    pub move_x: Option<i32>,
    pub move_y: Option<i32>,
}

pub struct Move {
    pub move_id: i32,
    pub game_id: i32,
    pub turn_number: i32,
    pub player: i32, // 1or2
    pub move_x: Option<i32>,
    pub move_y: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
}