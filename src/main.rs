mod book;
mod db;
use anyhow::Result;
use db::*;

fn main() -> Result<()> {
    let db = Db::from_file("test.json")?;

    for book in db.find().iter() {
        println!("{:?}", book)
    }
    Ok(())
}
