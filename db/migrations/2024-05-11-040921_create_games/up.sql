-- Your SQL goes here
CREATE TABLE games (
    game_id SERIAL PRIMARY KEY,
    player1_id INT NOT NULL,
    player2_id INT NOT NULL,
    current_turn INT CHECK (current_turn IN (1,2)) NOT NULL, -- 1 for Player1, 2 for Player2
    status VARCHAR(20) DEFAULT 'ongoing' CHECK (status IN ('ongoing', 'finished')) NOT NULL,
    black_board VARCHAR(64) NOT NULL, -- '0111..'のように要素64の1次元配列で --
    white_board VARCHAR(64) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    finished_at TIMESTAMP,
    FOREIGN KEY (player1_id) REFERENCES users(id),
    FOREIGN KEY (player2_id) REFERENCES users(id)
);