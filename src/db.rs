use std::time::Duration;

use crate::book::*;
use anyhow::{Context, Result};
use tokio::fs;
use tokio::time::sleep;

pub struct Db {
    arr: Vec<Book>,
    current_id: BookID,
}

#[allow(dead_code)]
impl Db {
    pub fn empty() -> Db {
        Db {
            arr: Vec::new(),
            current_id: BookID(1),
        }
    }
    pub async fn from_file(path: &str) -> Result<Db> {
        let mut db = Db::empty();
        let contents = fs::read_to_string(path)
            .await
            .context(format!("Failed to read {}", path))?;
        let books: Vec<BookDTO> =
            serde_json::from_str(&contents).context(format!("Failed to parse {}", path))?;
        for book in books.into_iter() {
            println!("Adding {}", book.name);
            db.add(book).await?;
        }
        Ok(db)
    }

    pub async fn add(&mut self, book: BookDTO) -> Result<()> {
        let new = Book::from_dto(self.next_id(), book);
        self.arr.push(new);
        sleep(Duration::from_secs(1)).await;
        Ok(())
    }
    pub async fn add_get(&mut self, book: BookDTO) -> Result<Book> {
        let new = Book::from_dto(self.next_id(), book);
        self.arr.push(new.clone());
        Ok(new)
    }
    pub async fn find(&self) -> &[Book] {
        self.arr.as_slice()
    }

    fn next_id(&mut self) -> BookID {
        self.current_id.0 += 1;
        self.current_id
    }
}
