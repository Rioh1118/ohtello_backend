use db::error::DbError;

mod error;
mod games_handler;
mod users_handler;
mod moves_handler;

fn main() -> Result<(), DbError> {
    // ゲーム開始。
    // プレイヤー1と2が決まる。
    // ゲームの状態が開始。game(初期状態)でデータベースに。
    // 合法手のリストを提供。
    // プレイヤー1が手を打つ。(PUTリクエスト)
    // ゲーム状態を更新。


    db::establish_connection()?;
    Ok(())
}
