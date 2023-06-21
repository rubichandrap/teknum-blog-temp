use crate::{
    components::{browse_topics::BrowseTopics, featured_post::FeaturedPost, layout::Layout},
    utils::dummy_creator::create_dummy_posts,
};
use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let posts = create_dummy_posts(10);
    let categories: Vec<String> = vec![
        "golang",
        "typescript",
        "rust",
        "keyboard",
        "random",
        "docker",
        "php",
        "gaming",
        "anime",
        "film",
        "twitter topics",
        "k8s",
        "react",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    view! { cx,
        <Layout>
            <FeaturedPost post=posts[0].clone()/>
            <BrowseTopics categories=categories/>
        </Layout>
    }
}
