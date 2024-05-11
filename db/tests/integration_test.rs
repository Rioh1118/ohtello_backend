use db;

#[test]
fn test_create_user() {
    let mut con = db::establish_connection().unwrap();
    let name = "test";
    let email = "example@gmail.com";
    let password = "password";

    let result = db::user::create_user(&mut con, name, email, password);

    match result {
        Ok(id) => assert_eq!(id, 1_i32),
        Err(e) => panic!("failed to create user : {:?}", e),
    }
}

#[test]
fn test_get_user_by_id() {
    let mut con = db::establish_connection().unwrap();
    let user_id = 1;

    let result = db::user::get_user_by_id(&mut con, user_id);

    match result {
        Ok(user) => assert_eq!(user.id, user_id),
        Err(e) => panic!("failed to get user by id : {:?}", e),
    }
}

#[test]
fn test_create_new_game() {
    let mut con = match db::establish_connection(){
        Ok(con) => con,
        Err(e) => panic!("failed to establish connection : {:?}", e),
    };
    let player1_id = 1;
    let player2_id = 4;
    let status = "ongoing";

    let result = db::game::create_new_game(&mut con, player1_id, player2_id, status);

    match result {
        Ok(game_id) => assert!(game_id > 0),
        Err(e) => panic!("failed to create new game : {:?}", e),
    }
}

#[test]
fn test_get_game_by_id() {
    let mut con = match db::establish_connection(){
        Ok(con) => con,
        Err(e) => panic!("failed to establish connection : {:?}", e),
    };
    let game_id = 1;

    let result = db::game::get_game_by_id(&mut con, game_id);

    match result {
        Ok(game) => assert_eq!(game.game_id, game_id),
        Err(e) => panic!("failed to get game by id : {:?}", e),
    }
}

#[test]
fn test_update_game_phase() {
    let mut con = match db::establish_connection(){
        Ok(con) => con,
        Err(e) => panic!("failed to establish connection : {:?}", e),
    };
    let game_id = 1;
    let black = "0000000000000000000000000000100000010000000000000000000000000000";
    let white = "0000000000000000000000000001000000001000000000000000000000000000";
    let next_turn = 2;
    let status = "ongoing";

    let result = db::game::update_game_phase(&mut con, game_id, black, white, next_turn, status);

    match result {
        Ok(_) => {
            let updated_game = db::game::get_game_by_id(&mut con, game_id).unwrap();
            assert_eq!(updated_game.black_board, black);
            assert_eq!(updated_game.white_board, white);
            assert_eq!(updated_game.current_turn, next_turn);
            assert_eq!(updated_game.status, status);
        },
        Err(e) => panic!("failed to update game phase : {:?}", e),
    }
}

#[test]
fn test_create_history() {
    let mut con = match db::establish_connection(){
        Ok(con) => con,
        Err(e) => panic!("failed to establish connection : {:?}", e),
    };
    let game_id = 1;
    let move_id = 1;

    let result = db::history::create_history(&mut con, game_id, move_id);

    match result {
        Ok(history_id) => assert!(history_id > 0),
        Err(e) => panic!("failed to create history : {:?}", e),
    }
}

#[test]
fn test_get_history_by_game_id() {
    let mut con = match db::establish_connection(){
        Ok(con) => con,
        Err(e) => panic!("failed to establish connection : {:?}", e),
    };
    let game_id = 1;

    let result = db::history::get_history_by_game_id(&mut con, game_id);

    match result {
        Ok(history_vec) => assert!(!history_vec.is_empty()),
        Err(e) => panic!("failed to get history by game id : {:?}", e),
    }
}
