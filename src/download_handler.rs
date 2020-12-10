use crate::post_handler;
use std::io;
use std::{fs::File, io::Write};


/// img object
pub struct Img {
    filename: String,
    filetype: String,
    url: String,
}

fn special_char_check(str_to_check: String) -> String {
    let special_chars = vec!["\\", "/", "\"", "?", ":", "*", "<", ">", "|"];
    let mut new_string = str_to_check;

    for char in special_chars {
        while new_string.contains(char) {
            new_string.remove(new_string.find(char).unwrap());
        }
    }
    return new_string;
}

/// creates a vectro with img objects from a vector of posts and returns it.
pub fn get_images(wanted_amount: usize, posts: &Vec<post_handler::Post>) -> Vec<Img> {
    let filetypes = vec!["png", "jpg", "gif"];

    let mut images: Vec<Img> = Vec::new();

    let mut image_count = 0;
    let mut post_index = 0;

    while image_count < wanted_amount {
        // f_index = filetype index
        for f_index in 0..filetypes.len() {
            // if link is not ending with (jpg, gif or png)
            // then check next

            let post = &posts[post_index];

            if post.post_url.split(".").last().unwrap().contains(filetypes[f_index])
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
pub fn download_text(wanted_amount: &usize, dest: &String, posts: &Vec<post_handler::Post>) {
    let mut text_count = 0;
    let mut index = 0;
    while text_count < wanted_amount {
        if posts[index].post_selftext == "" {
            println!("no self text")
        } else {
            println!(
                "{}{}.txt",
                dest,
                special_char_check(posts[index].post_title.clone())
            );

            let mut out = File::create(format!(
                "{}{}.txt",
                dest,
                special_char_check(posts[index].post_title.clone())
            ))
            .expect("could not create file");
            let text = posts[index].post_selftext.as_str().as_bytes();

            out.write_all(text);

            text_count += 1;
        }
        index += 1;
    }
}
