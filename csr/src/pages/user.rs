use leptos::{component, view, IntoView};

#[component]
pub fn User() -> impl IntoView {
    view! {
        <main class="p-4">
            <h1>"User"</h1>
            <p>"This is the user page"</p>
        </main>
    }
}
