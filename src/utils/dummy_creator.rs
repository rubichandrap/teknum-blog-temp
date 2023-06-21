use crate::utils::types::{Author, Post};

pub fn create_dummy_posts(total: usize) -> Vec<Post> {
    let author = Author {
        name: String::from("Rubi Chandraputra"),
        github: String::from("rubichandrap"),
        twitter: String::from("google.com"),
        telegram: String::from("google.com"),
    };

    let categories = vec![String::from("golang")];

    let posts = (0..total)
        .map(|_| Post {
            author: author.clone(),
            slug: String::from("rust-101"),
            content: String::from("test"),
            title: String::from("Berkenalan Dengan Laravel Scout"),
            desc: String::from("Membuat searching data makin mudah dan cepat (udah kayak tagline isp ga tuh)"),
            date: String::from("01/06/2023"),
            cover: String::from("https://laravelnews.s3.amazonaws.com/images/laravel-scout-featured.png?w=1366&h=698.52272727273&q=90&auto=format&fit=crop"),
            categories: categories.clone(),
        })
        .collect();

    posts
}