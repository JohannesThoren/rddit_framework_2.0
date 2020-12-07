pub struct Post {
    pub post_title: String,
    pub post_url: String,
    pub post_permalink: String,
    pub post_author: String,
    pub post_selftext: String
}

impl Post {
    pub fn new() -> Post {
        Post {
            post_title: String::new(),
            post_url: String::new(),
            post_permalink: String::new(),
            post_author: String::new(),
            post_selftext: String::new()
        }
    }
}