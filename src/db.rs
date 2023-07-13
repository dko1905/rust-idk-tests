use crate::book::{Book, BookDTO};
use anyhow::{Context, Result};

pub struct Db {
    arr: Vec<Book>,
    next_id: u32,
}

#[allow(dead_code)]
impl Db {
    pub fn empty() -> Db {
        Db {
            arr: Vec::new(),
            next_id: 1,
        }
    }
    pub fn from_file(path: &str) -> Result<Db> {
        let mut db = Db::empty();
        let contents = std::fs::read_to_string(path).context(format!("Failed to read {}", path))?;
        let books: Vec<BookDTO> =
            serde_json::from_str(&contents).context(format!("Failed to parse {}", path))?;
        for book in books.into_iter() {
            db.add(book)?;
        }
        Ok(db)
    }

    pub fn add(&mut self, book: BookDTO) -> Result<()> {
        let new = Book::from_dto(self.next_id, book);
        self.next_id += 1;
        self.arr.push(new);
        Ok(())
    }
    pub fn add_get(&mut self, book: BookDTO) -> Result<Book> {
        let new = Book::from_dto(self.next_id, book);
        self.next_id += 1;
        self.arr.push(new.clone());
        Ok(new)
    }
    pub fn find(&self) -> &[Book] {
        self.arr.as_slice()
    }
}
