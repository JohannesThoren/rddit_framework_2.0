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

use download_handler::download_text;

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





#[test]
fn test_img_dl() {
    let mut settings = url_handler::Settings::new();
    settings.subreddit = String::from("memes");
    settings.limit = 100;

    let posts = post_handler::get_data(&mut settings);
    let imgs = download_handler::get_images(10, &posts);
    download_handler::download_imgs(&imgs, &String::from("imgs/"));

}
#[test]
fn test_txt_dl() {
    let mut settings = url_handler::Settings::new();
    settings.subreddit = String::from("sverige");
    settings.limit = 100;

    let posts = post_handler::get_data(&mut settings);
    download_text(10, &String::from("text/"), &posts)

}
#[test]
fn test_search() {
    let mut settings = url_handler::Settings::new();
    settings.subreddit = String::from("sverige");
    settings.limit = 100;

    let posts = post_handler::get_data(&mut settings);
    let result = post_handler::search_post(posts.clone(), &mut String::from("nyheter"));

    for post in result{
        println!("{} | {}", post.post_author, post.post_title)
    }
}
#[test]
fn test_seacrh_dl_img() {
    let mut settings = url_handler::Settings::new();
    settings.subreddit = String::from("sweden");
    settings.limit = 100;
    settings.sorting = String::from("new");

    let mut posts = post_handler::get_data(&mut settings);
    let result = post_handler::search_post(posts.clone(), &mut String::from("Tack"));

    for post in &result{
        println!("{} | {}", post.post_author, post.post_title)
    }


    let imgs = download_handler::get_images(1, &result);
    download_handler::download_imgs(&imgs, &String::from("imgs/"))
}
