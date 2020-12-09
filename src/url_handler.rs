pub struct Settings {
    pub timespan: String,
    pub sorting: String,
    pub subreddit: String,
    pub limit: usize,
}
/// new settings object with standard values
impl Settings {
    pub fn new() -> Settings {
        Settings {
            timespan: String::from("day"),
            sorting: String::from("hot"),
            subreddit: String::from("memes"),
            limit: 10,
        }
    }
}

/// creates the url requierd to get data from reddit
pub fn get_url(settings: &Settings) -> String {
    format!(
        "https://www.reddit.com/r/{}/{}.json?t={}&limit={}",
        settings.subreddit,
        settings.sorting,
        settings.timespan,
        settings.limit.to_string()
    )
}