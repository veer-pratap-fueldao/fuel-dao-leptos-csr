use leptos::{component, view, IntoView};
use leptos_router::{Route, Router, Routes, A};

use crate::pages::{app::App, user::User};

#[component]
pub fn R() -> impl IntoView {
    view! {
        <Router>
            <nav class="flex gap-4">
                <A href="/">"Home"</A>
                <A href="/user" >"User"</A>
            </nav>

            <Routes>
                <Route path="/" view=App />
                <Route path="/user" view=User />
                <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
            </Routes>

            <footer>
                <p>"This is the footer"</p>
            </footer>
        </Router>
    }
}
