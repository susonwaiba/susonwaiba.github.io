use crate::page::about_page::*;
use crate::page::home_page::*;
use crate::page::note_page::*;
use crate::page::notes_page::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod api;
mod model;
mod page;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! { cx,
        <Router>
            <header class="app-header p-1">
                <div class="app-navbar">
                    <A href="/" class="nav-link app-btn">"Home"</A>
                    <A href="/about" class="nav-link app-btn">"About"</A>
                    <A href="/notes" class="nav-link app-btn">"Notes"</A>
                </div>
            </header>
            <main class="app-main">
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage /> }/>
                    <Route path="/about" view=|cx| view! { cx, <AboutPage /> }/>
                    <Route path="/notes" view=|cx| view! { cx, <NotesPage /> }/>
                    <Route path="/notes/:path" view=|cx| view! { cx, <NotePage /> }/>
                </Routes>
            </main>
            <footer class="app-footer">
                <p>"Copyright © Suson Waiba. All Rights Reserved."</p>
            </footer>
        </Router>
    }
}
