#[derive(Clone)]
pub struct Author {
    pub name: String,
    pub github: String,
    pub twitter: String,
    pub telegram: String,
}

#[derive(Clone)]
pub struct Post {
    pub author: Author,
    pub slug: String,
    pub content: String,
    pub title: String,
    pub desc: String,
    pub date: String,
    pub cover: String,
    pub categories: Vec<String>,
}
