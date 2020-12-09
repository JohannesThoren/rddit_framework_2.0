use crate::url_handler;
use serde_json;
pub struct Post {
    pub post_title: String,
    pub post_url: String,
    pub post_permalink: String,
    pub post_author: String,
    pub post_selftext: String,
}
/// new post object thats empty
impl Post {
    pub fn new() -> Post {
        Post {
            post_title: String::new(),
            post_url: String::new(),
            post_permalink: String::new(),
            post_author: String::new(),
            post_selftext: String::new(),
        }
    }
}

fn get_json(url: &str) -> serde_json::Value {
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    let data = serde_json::from_str(body.as_str()).expect("could not parse data from source");
    return data;
}
/// gets all posts and theire data and returns the posts as a vector
pub fn get_all_post_data(settings: &url_handler::Settings) -> Vec<Post> {
    let mut posts: Vec<Post> = Vec::new();

    let url = url_handler::get_url(&settings);

    let json_data = get_json(&url);

    for index in 0..settings.limit {
        let mut post = Post::new();

        // json parsing.  getting all data requierd
        // getting: author, permalink, title, url, selftext
        // pusing the data to the posts vec as a post object

        post.post_author = match &json_data["data"]["children"][index]["data"]["author"] {
            serde_json::Value::String(value) => value.clone(),
            _ => String::new(),
        };
        post.post_permalink = match &json_data["data"]["children"][index]["data"]["permalink"] {
            serde_json::Value::String(value) => value.clone(),
            _ => String::new(),
        };
        post.post_title = match &json_data["data"]["children"][index]["data"]["title"] {
            serde_json::Value::String(value) => value.clone(),
            _ => String::new(),
        };

        post.post_url = match &json_data["data"]["children"][index]["data"]["url"] {
            serde_json::Value::String(value) => value.clone(),
            _ => String::new(),
        };

        post.post_selftext = match &json_data["data"]["children"][index]["data"]["selftext"] {
            serde_json::Value::String(value) => value.clone(),
            _ => String::new(),
        };

        posts.push(post);
    }

    return posts;
}
