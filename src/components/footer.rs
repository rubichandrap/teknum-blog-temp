use chrono::Datelike;
use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    let current_date = chrono::Utc::now();
    let year = current_date.year();

    view! { cx,
        <footer class="mt-12 px-4 py-10 text-center border-t-2 border-gray-200 font-serif bg-white dark:bg-neutral-900 dark:text-neutral-100 dark:border-t-0  print:hidden">
            <p class="text-base pb-2">{format!("\u{00A9} {year} Teknologi Umum")}</p>
            <p class="text-xs">
                "Blog posts are licensed under "
                <a
                    href="https://github.com/teknologi-umum/blog/blob/master/LICENSE.CC-BY-SA-4.0"
                    class="text-primary-600 dark:text-primary-200 hover:underline"
                >
                    " Creative Commons Attribution Share Alike 4.0 International"
                </a> "."
            </p>
            <p class="text-xs">
                "Source code are licensed under "
                <a
                    href="https://github.com/teknologi-umum/blog/blob/master/LICENSE.GPL-3.0"
                    class="text-primary-600 dark:text-primary-200 hover:underline"
                >
                    "GNU General Public License v3.0"
                </a> "."
            </p>
        </footer>
    }
}
