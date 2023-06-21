use leptos::*;

#[component]
pub fn Twitter(
    cx: Scope,
    #[prop(default = "1rem".to_string())] width: String,
    #[prop(default = "1rem".to_string())] height: String,
) -> impl IntoView {
    view! { cx,
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=width
            height=height
            preserveAspectRatio="xMidYMid meet"
            viewBox="0 0 24 24"
        >
            <path
                fill="currentColor"
                d="M8.08 20A11.07 11.07 0 0 0 19.52 9A8.09 8.09 0 0 0 21 6.16a.44.44 0 0 0-.62-.51a1.88 1.88 0 0 1-2.16-.38a3.89 3.89 0 0 0-5.58-.17A4.13 4.13 0 0 0 11.49 9C8.14 9.2 5.84 7.61 4 5.43a.43.43 0 0 0-.75.24a9.68 9.68 0 0 0 4.6 10.05A6.73 6.73 0 0 1 3.38 18a.45.45 0 0 0-.14.84A11 11 0 0 0 8.08 20"
            ></path>
        </svg>
    }
}
