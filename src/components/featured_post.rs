use crate::utils::types::Post;
use leptos::*;
use leptos_router::*;

#[component]
pub fn FeaturedPost(cx: Scope, post: Post) -> impl IntoView {
    view! { cx,
        <div class="grid lg:grid-cols-2 grid-cols-1 items-center mt-12 md:-mx-12 lg:gap-x-20 gap-y-10 lg:gap-y-0">
            <div class="lg:text-left text-center">
                <p class="text-black dark:text-neutral-100 font-medium uppercase">"featured"</p>
                <a href=format!("/posts/{}", post.slug)>
                    <h1 class="sm:text-5xl text-3xl mb-4 mt-2 font-bold text-black dark:text-neutral-100">
                        {&post.title}
                    </h1>
                </a>
                <a
                    class="group inline-block"
                    href=format!("https://github.com/{}", post.author.github)
                >
                    <div class="flex space-x-2 items-center justify-start">
                        <img
                            class="rounded-full shadow-md"
                            src=format!("https://github.com/{}.png", post.author.github)
                            width="32"
                            height="32"
                            alt=&post.author.github
                        />
                        <span class="text-sm text-left text-gray-700 group-hover:text-primary-600 dark:text-neutral-100 dark:group-hover:text-primary-200">
                            {&post.author.name}
                        </span>
                    </div>
                </a>
                <p class="mt-4 leading-relaxed font-serif text-gray-500 dark:text-gray-100">
                    {&post.desc}
                </p>
                <A
                    class="bg-primary-900 hover:bg-primary-700 dark:bg-primary-500 dark:hover:bg-primary-300 inline-block text-white dark:hover:text-black mt-4 py-1.5 px-10 transition duration-300 print:hidden uppercase"
                    href=format!("/posts/{}", post.slug)
                >
                    "read more"
                </A>
            </div>
            <div class="w-5/6 lg:w-full mx-auto lg:mx-0">
                <div class="aspect-w-16 aspect-h-13 w-full">
                    <img class="w-full object-cover object-center" src=&post.cover alt=&post.title/>
                </div>
            </div>
        </div>
    }
}
