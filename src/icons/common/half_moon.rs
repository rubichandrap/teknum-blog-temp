use leptos::*;

#[component]
pub fn HalfMoon(
    cx: Scope,
    #[prop(default = "1rem".to_string())] width: String,
    #[prop(default = "1rem".to_string())] height: String,
) -> impl IntoView {
    view! { cx,
        <svg
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 512 512"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            enable-background="new 0 0 512 512"
            width=width
            height=height
        >
            <g>
                <path
                    fill="currentColor"
                    d="m256,11c-135.3,0-245,109.7-245,245s109.7,245 245,245 245-109.7 245-245-109.7-245-245-245zm-204,245c0-112.5 91.5-204 204-204 12,0 23.8,1.1 35.3,3.1-63.4,44.2-104.8,117.7-104.8,200.9 0,83.2 41.5,156.7 104.8,200.9-11.5,2-23.3,3.1-35.3,3.1-112.5,0-204-91.5-204-204zm291.7,184.1c-68.7-32.8-116.2-103-116.2-184.1s47.5-151.3 116.2-184.1c68.7,32.8 116.3,103 116.3,184.1s-47.6,151.3-116.3,184.1z"
                ></path>
            </g>
        </svg>
    }
}