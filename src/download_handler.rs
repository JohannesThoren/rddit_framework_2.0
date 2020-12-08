use crate::post_handler;

pub fn get_images(count: usize, posts: Vec<post_handler::Post>) {
    let filetypes = vec!["png", "jpg", "gif"];
    // if link is not ending with (jpg, gif or png)
    // then check next
    let mut i = 0;
    while i < count {
        println!("{}:x", i);

        for filetype in &filetypes {
            if posts[i].post_url.as_str().ends_with(filetype) {
                println!("{}", posts[i].post_url);
                println!("{}",i);
                i += 1;
            }
        }
    }
}
