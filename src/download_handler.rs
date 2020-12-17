use crate::post_handler;
use htmlescape::decode_html;
use std::io;
use std::{fs::File, io::Write};

/// img object
pub struct Img {
    filename: String,
    filetype: String,
    url: String,
}

fn shorten(str_to_shorten: &String) -> String {
    let mut new_str = str_to_shorten.clone();
    if str_to_shorten.len() >= 64 {
        new_str = String::from(str_to_shorten.split_at(64).0);
        // println!("{}", new_str)
    }
    // println!("{}", new_str);
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

/// creates a vectro with img objects from a vector of posts and returns it.
pub fn get_images(wanted_amount: usize, posts: &Vec<post_handler::Post>) -> Vec<Img> {
    let filetypes = vec!["png", "jpg", "gif"];

    let mut images: Vec<Img> = Vec::new();

    let mut image_count = 0;
    let mut post_index = 0;

    while image_count < wanted_amount && post_index < posts.len() {
        // f_index = filetype index
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
/// Downloads the selftext from a post if there are any
pub fn download_text(wanted_amount: usize, dest: &String, posts: &Vec<post_handler::Post>) {
    let mut text_count = 0;
    let mut post_index = 0;

    // some css to make the text look nice
    let style = String::from(
        r"
        <style>
        body {
            width: 21cm;
            min-height: 29.7cm;
            height: fit-content;
            height: -moz-fit-content;
            height: -webkit-fit-content;

            margin-left: auto;
            margin-right: auto;

            box-shadow: 0 0.25rem 0.5rem 0 black;

            padding: 0.625rem;
        }

        h1 {
            text-align: center;
            text-decoration: underline;
        }

       @media print {
           body {
            box-shadow: none;
           }
        }
        </style>
    ",
    );

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
                "<head>{}</head><body><h1>{}</h1>{}</body>",
                style,
                title,
                decode_html(posts[post_index].post_selftext.as_str()).unwrap()
            );

            out.write_all(text.as_bytes());

            text_count += 1;
        }
        post_index += 1;
    }
}
