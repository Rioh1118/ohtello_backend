-- Your SQL goes here
CREATE TABLE histories (
    history_id SERIAL PRIMARY KEY,
    game_id INT NOT NULL,
    move_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (game_id) REFERENCES games(game_id),
    FOREIGN KEY (move_id) REFERENCES moves(move_id)
);