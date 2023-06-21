use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use leptos::*;

#[component]
pub fn Layout(cx: Scope, children: Children) -> impl IntoView {
    let children = children(cx)
        .nodes
        .into_iter()
        .map(|child| view! { cx, <>{child}</> })
        .collect_view(cx);

    view! { cx,
        <div class="bg-white dark:bg-neutral-900 ">
            <Navbar/>
            <div class="container mx-auto pt-10 lg:pt-2 w-full font-sans print:pt-0">
                <main class="flex flex-col pt-16 px-8 sm:px-16 md:px-32 lg:px-40 xl:px-56 2xl:px-72 print:pt-0">
                    <div class="flex-1 h-full min-h-screen mt-8 md:mt-0">{children}</div>
                    <Footer/>
                </main>
            </div>
        </div>
    }
}
