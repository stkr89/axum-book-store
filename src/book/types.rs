use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct CreateBookRequest {
    #[validate(length(min = 5))]
    pub title: String,
    #[validate(length(min = 5))]
    pub author: String,
    #[validate(range(min = 100))]
    pub pages: i16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BookResponse {
    pub id: String,
    pub title: String,
    pub author: String,
    pub pages: i16,
}