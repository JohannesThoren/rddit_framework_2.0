/*
 *   Copyright (c) 2021 Johannes Thorén
 *   All rights reserved.

 *   Permission is hereby granted, free of charge, to any person obtaining a copy
 *   of this software and associated documentation files (the "Software"), to deal
 *   in the Software without restriction, including without limitation the rights
 *   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *   copies of the Software, and to permit persons to whom the Software is
 *   furnished to do so, subject to the following conditions:
 
 *   The above copyright notice and this permission notice shall be included in all
 *   copies or substantial portions of the Software.
 
 *   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *   SOFTWARE.
 */

use crate::url;
use serde_json;
#[derive(Debug, Clone)]
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
pub fn data(settings: &mut url::Settings) -> Vec<Post> {
    let mut posts: Vec<Post> = Vec::new();

    let url = url::get_url(settings);

    let children = &get_json(&url)["data"]["children"];


    // so basically  "children.as_array().unwrap().len()" creates an array of all children, unwraps it and gets the len
    // so i can iterate over the len.
    // maybe a bodge but it works.
    for index in 0..children.as_array().unwrap().len() {
        let mut post = Post::new();

        // json parsing.  getting all data requirer
        // getting: author, permalink, title, url, selftext
        // pusing the data to the posts vec as a post object

        post.post_author = match &children[index]["data"]["author"] {
            serde_json::Value::String(value) => value.clone(),
            _ => String::new(),
        };
        post.post_permalink = match &children[index]["data"]["permalink"] {
            serde_json::Value::String(value) => value.clone(),
            _ => String::new(),
        };
        post.post_title = match &children[index]["data"]["title"] {
            serde_json::Value::String(value) => value.clone(),
            _ => String::new(),
        };

        post.post_url = match &children[index]["data"]["url"] {
            serde_json::Value::String(value) => value.clone(),
            _ => String::new(),
        };

        post.post_selftext = match &children[index]["data"]["selftext_html"] {
            serde_json::Value::String(value) => value.clone(),
            _ => String::new(),
        };

        posts.push(post);
    }
    return posts;
}


pub fn search_post(posts: Vec<Post>, keyword: &mut String) -> Vec<Post>{

    let mut result = Vec::new();

    for post in  posts{
        if post.post_title.contains(keyword.as_str()) {
            result.push(post)
        }
    }
    return result;

}
