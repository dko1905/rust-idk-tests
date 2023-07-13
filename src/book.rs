use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BookDTO {
    name: String,
    year: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Book {
    id: u32,
    name: String,
    year: u32,
}

impl Book {
    pub fn from_dto(id: u32, book: BookDTO) -> Book {
        Book {
            id: id,
            name: book.name,
            year: book.year,
        }
    }
}
