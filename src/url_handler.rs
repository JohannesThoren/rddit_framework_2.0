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

pub struct Settings {
    pub timespan: String,
    pub sorting: String,
    pub subreddit: String,
//    pub limit: usize,
}
/// new settings object with standard values
impl Settings {
    pub fn new() -> Settings {

        //setting some standard settings
        Settings {
            timespan: String::from("day"),
            sorting: String::from("hot"),
            subreddit: String::from("memes"),

        }
    }
}

/// creates the url requierd to get data from reddit
pub fn get_url(settings: &mut Settings) -> String {


    // removed limit, max limit of reddit is 100 so i now use 100 as limit
    format!(
        "https://www.reddit.com/r/{}/{}.json?t={}&limit=100",
        settings.subreddit,
        settings.sorting,
        settings.timespan,                                         
    )                                
}
