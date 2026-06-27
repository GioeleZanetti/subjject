use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    StaticSegment, WildcardSegment, components::{A, ParentRoute, Route, Router, Routes}, path
};
use crate::{components::topbar::*, server::get_home_stats};
use crate::components::navbar::*;
use crate::components::moves::*;


#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/subjject.css"/>

        // sets the document title
        <Title text="Subjject"/>

        <Topbar />
        <div class="main-content">
            <Router>
                <Navbar />
                <div class="content">
                    <main>
                        <Routes fallback=move || "Not found.">
                            <ParentRoute path=path!("/techniques") view=MoveLibrary>
                                <Route path=path!(":id") view=MoveDetail/>
                                <Route path=path!("") view=NoMoveSelected/>
                            </ParentRoute>
                            <Route path=StaticSegment("") view=HomePage/>
                            <Route path=WildcardSegment("any") view=NotFound/>
                        </Routes>
                    </main>
                </div>
            </Router>
        </div>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    
    let stats_resource = Resource::new(
        || (), 
        |_| async move { get_home_stats().await.unwrap_or_default() }
    );

    view! {
        <div class="home-container">
            
            <div class="hero-section">
                <div class="hero-text">
                    <h2 class="hero-title">"Master Your System."</h2>
                    <p class="hero-subtitle">"Track your grappling techniques, study your notes, and build an unbreakable game plan."</p>
                </div>
                <A href="/techniques" attr:class="btn-primary">"Browse Library"</A>
            </div>

            <Suspense fallback=|| view! { <p class="loading">"Loading stats..."</p> }>
                {move || stats_resource.get().map(|stats| view! {
                    <div class="stats-grid">
                        <div class="stat-card">
                            <div class="stat-icon total-icon">
                                <svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 6.042A8.967 8.967 0 006 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 016 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 016-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0018 18a8.967 8.967 0 00-6 2.292m0-14.25v14.25" />
                                </svg>
                            </div>
                            <div class="stat-info">
                                <div class="stat-value">{stats.total}</div>
                                <div class="stat-label">"Total Techniques"</div>
                            </div>
                        </div>

                        <div class="stat-card">
                            <div class="stat-icon sub-icon">
                                <svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 13.5l10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75z" />
                                </svg>
                            </div>
                            <div class="stat-info">
                                <div class="stat-value">{stats.submissions}</div>
                                <div class="stat-label">"Submissions"</div>
                            </div>
                        </div>

                        <div class="stat-card">
                            <div class="stat-icon takedown-icon">
                                <svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 13.5L12 21m0 0l-7.5-7.5M12 21V3" />
                                </svg>
                            </div>
                            <div class="stat-info">
                                <div class="stat-value">{stats.takedowns}</div>
                                <div class="stat-label">"Takedowns"</div>
                            </div>
                        </div>

                        <div class="stat-card">
                            <div class="stat-icon guard-icon">
                                <svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75m-3-7.036A11.959 11.959 0 013.598 6 11.99 11.99 0 003 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285z" />
                                </svg>
                            </div>
                            <div class="stat-info">
                                <div class="stat-value">{stats.guards}</div>
                                <div class="stat-label">"Guard"</div>
                            </div>
                        </div>

                        <div class="stat-card">
                            <div class="stat-icon pass-icon">
                                <svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M13.5 4.5L21 12m0 0l-7.5 7.5M21 12H3" />
                                </svg>
                            </div>
                            <div class="stat-info">
                                <div class="stat-value">{stats.passes}</div>
                                <div class="stat-label">"Passes"</div>
                            </div>
                        </div>

                        <div class="stat-card">
                            <div class="stat-icon escape-icon">
                                <svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15m3 0l3-3m0 0l-3-3m3 3H9" />
                                </svg>
                            </div>
                            <div class="stat-info">
                                <div class="stat-value">{stats.escapes}</div>
                                <div class="stat-label">"Escapes"</div>
                            </div>
                        </div>

                    </div>
                })}
            </Suspense>

            <div class="focus-section">
                <div class="focus-card">
                    <div class="focus-icon-large">
                        <svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 13.5l10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75z" />
                        </svg>
                    </div>
                    <div class="focus-content">
                        <h3>"Focus on the Fundamentals"</h3>
                        <p>"Consistency is the key to mastery. Review your study notes before class, drill your weak spots purposefully, and stay dangerous on the mats."</p>
                    </div>
                </div>
            </div>

        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <div class="not-found-container">
            <div class="not-found-content">
                <div class="error-code">"404"</div>
                
                <h2>"Lost on the Mats?"</h2>
                <p>"The page or technique you are looking for doesn't exist, or it might have been moved."</p>
                
                <A href="/" attr:class="btn-primary">"Back to Homepage"</A>
            </div>
        </div>
    }
}
