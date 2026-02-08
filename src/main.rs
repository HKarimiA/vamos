use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

mod core;
mod pages;

use pages::{Grammar, Home, Vocabulary};

fn main() {
    leptos::mount::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Page not found">
                <Route path=path!("/") view=Home/>
                <Route path=path!("/vocabulary") view=Vocabulary/>
                <Route path=path!("/grammar") view=Grammar/>
            </Routes>
        </Router>
    }
}
