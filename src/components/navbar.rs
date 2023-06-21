use crate::icons::{
    brand::{github::Github, telegram::Telegram},
    common::search::Search,
};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    let routes: Vec<String> = vec!["home", "blog", "about"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    view! { cx,
        <nav class="fixed min-h-16 top-0 inset-x-0 bg-white dark:bg-neutral-800 bg-opacity-80 backdrop-filter backdrop-blur-lg z-20 font-sans print:static">
            <div class="container flex flex-col md:flex-row justify-between items-center space-y-1 mx-auto py-5 px-8 sm:px-16 md:px-32 lg:px-40 xl:px-56 2xl:px-72 h-full">
                <div class="flex-1 space-x-6 print:hidden">
                    {routes
                        .into_iter()
                        .map(|route| {
                            view! { cx,
                                <Route
                                    label=route.clone()
                                    href=if route == "home".to_string() { "".to_string() } else { route.to_string() }
                                />
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
                <div class="flex-2 lg:flex-1 text-center dark:text-neutral-100 text-3xl mb-2 lg:mb-0">
                    <A href="" class="main-logo">
                        "Teknologi Umum"
                    </A>
                </div>
                <div class="flex-1 text-right text-lg print:hidden">
                    <div class="flex flex-row items-center justify-end space-x-4">
                        <div class="flex-initial opacity-60 hover:text-primary-600/100 dark:text-neutral-300 dark:hover:text-neutral-50 transition duration-300">
                            <a href="/search">
                                <Search width="1.5rem".to_string() height="1.5rem".to_string()/>
                            </a>
                        </div>
                        <div class="flex-initial opacity-60 hover:text-primary-600/100 dark:text-neutral-300 dark:hover:text-neutral-50 transition duration-300 transition duration-300">
                            <a href="https://github.com/teknologi-umum">
                                <Github width="1.5rem".to_string() height="1.5rem".to_string()/>
                            </a>
                        </div>
                        <div class="flex-initial opacity-60 hover:text-primary-600/100 dark:text-neutral-300 dark:hover:text-neutral-50 transition duration-300 transition duration-300">
                            <a href="https://t.me/teknologi_umum">
                                <Telegram width="2rem".to_string() height="2rem".to_string()/>
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn Route(cx: Scope, label: String, href: String) -> impl IntoView {
    view! { cx,
        <A
            href=href
            class="flex-inline uppercase text-center hover:text-primary-600 dark:text-neutral-300 dark:hover:text-neutral-50 transition duration-300"
        >
            {label}
        </A>
    }
}
