/*
*   Copyright (c) 2020 Johannes Thorén
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

use crate::post_handler;
use htmlescape::decode_html;
use std::io;
use std::{fs::*, io::Write};

/// img object
pub struct Img {
    filename: String,
    filetype: String,
    pub url: String,
}

fn shorten(str_to_shorten: &String) -> String {
    let mut new_str = str_to_shorten.clone();
    let chars: Vec<char> = str_to_shorten.chars().collect();

    if chars.len() > 64 {
        new_str = chars[0..64].iter().collect();

        println!("{}", new_str)
    }
    return new_str;
}

fn special_char_check(str_to_check: String) -> String {
    let special_chars = vec!["\\", "/", "\"", "?", ":", "*", "<", ">", "|"];
    let mut new_string = str_to_check;

    for char in special_chars {
        while new_string.contains(char) {
            new_string.remove(new_string.find(char).unwrap());
        }
    }
    return shorten(&new_string);
}

/// creates a vector with img objects from a vector of posts and returns it.
pub fn img_data(wanted_amount: usize, posts: &Vec<post_handler::Post>) -> Vec<Img> {
    let filetypes = vec!["png", "jpg", "gif"];

    let mut images: Vec<Img> = Vec::new();

    let mut image_count = 0;
    let mut post_index = 0;

    while image_count < wanted_amount && post_index < posts.len() {
        for f_index in 0..filetypes.len() {
            // if link is not ending with (jpg, gif or png)
            // then check next

            let post = &posts[post_index];

            if post
                .post_url
                .split(".")
                .last()
                .unwrap()
                .contains(filetypes[f_index])
                && post.post_url.split(".").last().unwrap().len() <= 3
            {
                let image = Img {
                    filename: special_char_check(post.post_title.clone()),
                    filetype: String::from(filetypes[f_index]),
                    url: post.post_url.clone(),
                };

                images.push(image);
                image_count += 1;
            }
        }
        post_index += 1;
    }

    return images;
}


// TODO see if it is possible to make this better
/// This should explain itself..... it downloads images.
/// create a vector and fill it with img objects.
pub fn download_imgs(imgs: &Vec<Img>, dest: &String) {
    for img in imgs {
        println!("{}{}.{}", dest, img.filename, img.filetype);

        let mut out = File::create(format!("{}{}.{}", dest, img.filename, img.filetype).as_str())
            .expect("could not create file");
        let mut data = reqwest::blocking::get(img.url.as_str()).expect("could not fetch data");

        io::copy(&mut data, &mut out).expect("could not write data to file");
    }
}

// BAD implementation of a text downloader XD
// TODO create a better way of downloading text
/// Downloads the selftext from a post if there are any
pub fn download_text(wanted_amount: usize, dest: &String, posts: &Vec<post_handler::Post>) {
    let mut text_count = 0;
    let mut post_index = 0;

    //get css style sheet from the style.css file
    let style = String::from(include_str!("style.css"));

    // TODO move all html code to somewhere
    while text_count < wanted_amount && post_index < posts.len() {
        if posts[post_index].post_selftext == "" {
            // println!("no self text")
        } else {
            println!(
                "{}{}.html",
                dest,
                special_char_check(posts[post_index].post_title.clone())
            );

            let mut out = File::create(format!(
                "{}{}.html",
                dest,
                special_char_check(posts[post_index].post_title.clone())
            ))
            .expect("could not create file");

            let title = &posts[post_index].post_title;
            let text = format!(
                "<head>\n<style>\n{}\n</style>\n</head>\n<body>\n<h1><a href = '{}'>{}</a></h1>\n{}\n</body>",
                style,
                posts[post_index].post_url,
                title,
                decode_html(posts[post_index].post_selftext.as_str()).unwrap()
            );

            out.write_all(text.as_bytes());

            text_count += 1;
        }
        post_index += 1;
    }
}
