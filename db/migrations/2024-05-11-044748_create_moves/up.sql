-- Your SQL goes here
CREATE TABLE moves(
    move_id SERIAL PRIMARY KEY,
    game_id INT NOT NULL,
    turn_number INT NOT NULL, -- このゲームの何ターン目か
    player INT NOT NULL, -- 1か2(1:先手, 2: 後手)
    move_x INT, -- パスの場合はNULL
    move_y INT, -- パスの場合はNULL
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (game_id) REFERENCES games(game_id)
);