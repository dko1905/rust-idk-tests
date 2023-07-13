mod book;
mod db;
use std::time::Duration;

use anyhow::Result;
use db::*;

#[tokio::main]
async fn main() -> Result<()> {
    let handle = tokio::spawn(async {
        for i in 0..10 {
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("Sleep {}", i)
        }
    });
    let db = Db::from_file("test.json").await?;
    handle.await?;

    for book in db.find().await.iter() {
        println!("{:?}", book)
    }
    Ok(())
}
