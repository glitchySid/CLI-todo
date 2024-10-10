mod database;
mod input;
mod app;
use rusqlite::Connection;
use rusqlite::Result;
use app::run;
fn main() ->Result<()>{
    let conn = Connection::open("data.db")?;
    run(&conn);
    Ok(())
}
