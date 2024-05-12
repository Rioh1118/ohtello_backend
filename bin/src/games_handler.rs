use actix_web::{get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewGame {
    pub player1_id: i32,
    pub player2_id: i32,
    pub status: String,
}
// _game_id: i32, black: &'a str, white: &'a str, next_turn: i32, _status: &'a str
#[derive(Deserialize)]
pub struct GameUpdate {
    pub black_board: String,
    pub white_board: String,
    pub next_turn: i32,
    pub status: String,
}
#[post("/games")]
pub async fn new_game(new_game: web::Json<NewGame>) -> impl Responder {
    let res_conn = db::establish_connection();

    match res_conn {
        Ok(mut conn) => {
            let new_game = db::game::create_new_game(
                &mut conn,
                new_game.player1_id,
                new_game.player2_id,
                &new_game.status,
            );
            match new_game {
                Ok(game_id) => HttpResponse::Ok().json(game_id),
                Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[get("games/{game_id}")]
pub async fn get_game_by_id(game_id: web::Path<i32>) -> HttpResponse {
    let res_conn = db::establish_connection();

    match res_conn {
        Ok(mut conn) => {
            let game_res = db::game::get_game_by_id(&mut conn, game_id.into_inner());
            match game_res {
                Ok(game) => HttpResponse::Ok().json(game),
                Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[put("games/{game_id}")]
pub async fn update_game(
    game_id: web::Path<i32>,
    game_update: web::Json<GameUpdate>,
) -> HttpResponse {
    let res_conn = db::establish_connection();

    match res_conn {
        Ok(mut conn) => {
            let result = db::game::update_game_phase(
                &mut conn,
                game_id.into_inner(),
                &game_update.black_board,
                &game_update.white_board,
                game_update.next_turn,
                &game_update.status,
            );
            match result {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[get("/users/{user_id}/games")]
pub async fn get_games_by_player_id(user_id: web::Path<i32>) -> impl Responder {
    let res_conn = db::establish_connection();

    match res_conn {
        Ok(mut conn) => {
            let games_res = db::game::get_games_by_player_id(&mut conn, user_id.into_inner());
            match games_res {
                Ok(games) => HttpResponse::Ok().json(games),
                Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
