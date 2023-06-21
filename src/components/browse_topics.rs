use crate::utils::constants::SHOWN_TOPICS_LIMIT;
use leptos::*;
use leptos_router::*;

#[component]
pub fn BrowseTopics(cx: Scope, categories: Vec<String>) -> impl IntoView {
    let limited_categories = categories
        .clone()
        .into_iter()
        .take(SHOWN_TOPICS_LIMIT)
        .collect::<Vec<String>>();
    let (show_more_topics, set_show_more_topics) = create_signal(cx, false);
    let (local_categories, set_local_categories) = create_signal(cx, limited_categories.clone());

    create_effect(cx, move |_| {
        if show_more_topics() {
            set_local_categories.update(|p| *p = categories.clone());
        } else {
            set_local_categories.update(|p| {
                *p = limited_categories.clone();
            });
        }
    });

    view! { cx,
        <div class="md:-mx-12 mt-24">
            <h2 class="uppercase font-bold text-xl mb-10 dark:text-neutral-100">
                "Browse Interesting Topics"
            </h2>
            <div class="flex flex-wrap gap-6 items-center justify-center">
                {move || {
                    local_categories
                        .get()
                        .into_iter()
                        .map(|category| {
                            view! { cx, <Category category=category/> }
                        })
                        .collect::<Vec<_>>()
                }}
            </div>
            <button
                class="block uppercase mx-auto mt-10 font-medium text-lg text-gray-600 hover:text-primary-600 dark:text-gray-200 dark:hover:text-primary-200 hover:underline decoration-dashed underline-offset-4"
                on:click=move |_| set_show_more_topics.update(|p| *p = !*p)
            >
                "show "
                {move || if show_more_topics.get() { "less" } else { "more" }}
                " topics"
            </button>
        </div>
    }
}

#[component]
fn Category(cx: Scope, category: String) -> impl IntoView {
    view! { cx,
        <A
            href=format!("/search?q={}", category)
            class="flex-[0_0_calc(20%-1.5rem)] text-lg pb-3 border-b border-black text-center lowercase hover:text-primary-600 dark:text-neutral-100 dark:hover:text-primary-200 whitespace-nowrap"
        >
            {category}
        </A>
    }
}
