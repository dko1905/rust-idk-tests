use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BookDTO {
    pub name: String,
    pub year: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Book {
    pub id: BookID,
    pub name: String,
    pub year: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BookID(pub u32);

impl Book {
    pub fn from_dto(id: BookID, book: BookDTO) -> Book {
        Book {
            id: id,
            name: book.name,
            year: book.year,
        }
    }
}
