use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

mod components;
mod core;
mod data;
mod pages;

use core::{FavoritesContext, LanguageContext};
use pages::{
    ExpressionCards, Expressions, Favorites, Grammar, GrammarTopic, Home, Settings, Vocabulary,
    VocabularyCards,
};

fn main() {
    leptos::mount::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    provide_context(FavoritesContext::new());
    provide_context(LanguageContext::new());

    view! {
        <Router>
            <Routes fallback=|| "Page not found">
                <Route path=path!("/") view=Home/>
                <Route path=path!("/settings") view=Settings/>
                <Route path=path!("/vocabulary") view=Vocabulary/>
                <Route path=path!("/vocabulary/favorites") view=Favorites/>
                <Route path=path!("/vocabulary/:stage") view=VocabularyCards/>
                <Route path=path!("/grammar") view=Grammar/>
                <Route path=path!("/grammar/:id") view=GrammarTopic/>
                <Route path=path!("/expressions") view=Expressions/>
                <Route path=path!("/expressions/:unit") view=ExpressionCards/>
            </Routes>
        </Router>
    }
}
