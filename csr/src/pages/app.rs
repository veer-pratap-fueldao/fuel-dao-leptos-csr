use leptos::{component, create_signal, view, IntoView, SignalUpdate};

use crate::components::shared::progress_bar::ProgressBar;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <main class="flex flex-col gap-4 p-4 w-fit">
            <h1 class="text-green">"Hello, World!"</h1>
            <p>{count}</p>
            <button
                // style:background-color=move || format!("rgb({}, {}, 100)", 100, 100)
                // class:red=move || count.get() % 2 == 1
                class="bg-black text-white p-2 rounded"
                on:click=move |_| {
                    set_count.update(|n| *n += 1);
                }
            >
                "Increment test: "
                {count}
            </button>

            <ProgressBar progress=count />
        </main>
    }
}

// fn handle_increment(count: ReadSignal<i32>, set_count: WriteSignal<i32>) {
//     set_count.set(count.get() + 1i32);
// }
