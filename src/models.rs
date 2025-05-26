use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogPost {
    id: i32,
    title: String,
    content: String,
    author: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewBlogPost {
    title: String,
    content: String,
    author: String,
}
