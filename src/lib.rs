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
pub mod post_handler;
pub mod url_handler;
pub mod download_handler;


#[cfg(test)]
#[test]
fn test() {
    let mut settings = url_handler::Settings::new();
    settings.subreddit = String::from("sverige");
    settings.sorting = String::from("new");
    settings.limit = 50;

    let posts = post_handler::get_all_post_data(&settings);

    // for post in &posts {
    //     println!("----------\ntitle : {}\nauthor : {}\npermalink : {}\nurl : {}\n----------\n\n",post.post_title, post.post_author, post.post_permalink, post.post_url)
    // }
    let imgs = download_handler::get_images(4, &posts);    
    download_handler::download_imgs(&imgs, String::from("test/"));
    download_handler::download_text(String::from("text/"), &posts);
}