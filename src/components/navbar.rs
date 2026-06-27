use leptos::prelude::*;
use leptos::{IntoView, component, view};
use leptos_router::components::A;

#[component]
pub fn Navbar() -> impl IntoView {
    let nav_links = vec![
        ("Homepage", "/", "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"),
        ("Techniques", "/techniques", "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"),
        ("Game Plan", "/gameplan", "M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"),
    ];

    view! {
        <nav class="navbar">
            <ul class="nav-list">
                {nav_links.into_iter().map(|(name, path, icon_path)| view! {
                    <li>
                        <A href=path exact=true attr:class="nav-item">
                            <svg class="nav-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                                <path stroke-linecap="round" stroke-linejoin="round" d=icon_path />
                            </svg>
                            <span class="nav-text">{name}</span>
                        </A>
                    </li>
                }).collect_view()}
            </ul>
        </nav>
    }
}