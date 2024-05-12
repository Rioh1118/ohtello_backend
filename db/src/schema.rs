// @generated automatically by Diesel CLI.

diesel::table! {
    games (game_id) {
        game_id -> Int4,
        player1_id -> Int4,
        player2_id -> Int4,
        current_turn -> Int4,
        #[max_length = 20]
        status -> Varchar,
        #[max_length = 64]
        black_board -> Varchar,
        #[max_length = 64]
        white_board -> Varchar,
        created_at -> Timestamp,
        finished_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    histories (history_id) {
        history_id -> Int4,
        game_id -> Int4,
        move_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    moves (move_id) {
        move_id -> Int4,
        game_id -> Int4,
        turn_number -> Int4,
        player -> Int4,
        move_x -> Nullable<Int4>,
        move_y -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(histories -> games (game_id));
diesel::joinable!(histories -> moves (move_id));
diesel::joinable!(moves -> games (game_id));

diesel::allow_tables_to_appear_in_same_query!(games, histories, moves, users,);
