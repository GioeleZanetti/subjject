use leptos::prelude::*;

#[component]
pub fn Topbar() -> impl IntoView {
    view! {
        <header class="topbar">
            <div class="topbar-left">
                <svg class="brand-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" />
                </svg>
                <h1 class="brand-name">"Subjject"</h1>
            </div>
        </header>
    }
}