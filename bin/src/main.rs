use db::error::DbError;
fn main() -> Result<(),DbError>{
    db::establish_connection()?;
    Ok(())
}
