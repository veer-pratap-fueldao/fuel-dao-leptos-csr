use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {

        <header class="bg-white p-4 shadow">
            <div class="container mx-auto flex justify-between items-center">
                <img src="/img/fueldao.svg" alt="Logo" class="h-8" />
                <nav class="flex space-x-4">
                    <a href="#" class="text-gray-700 hover:text-green-500">"List your property"</a>
                    <a href="#" class="text-gray-700 hover:text-green-500">"Support"</a>
                    <a href="#" class="text-gray-700 hover:text-green-500">"Trips"</a>
                    <a href="#" class="text-gray-700 hover:text-green-500">"Sign in"</a>
                    <button class="bg-green-500 text-white px-4 py-2 rounded">"Get the app"</button>
                </nav>
            </div>
        </header>
    }
}
