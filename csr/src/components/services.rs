use leptos::*;

#[component]
pub fn Services() -> impl IntoView {
    view! {
        <section class="bg-white p-12">
            <div class="container mx-auto text-center">
                <h2 class="text-3xl font-bold mb-8">"Our Services"</h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                    <div class="bg-gray-100 p-6 rounded-lg shadow-lg">
                        <h3 class="text-xl font-bold mb-4">"Instant Rent"</h3>
                        <p>"We provide instant car rental services..."</p>
                        <button class="mt-4 bg-green-500 text-white px-4 py-2 rounded">"Read More"</button>
                    </div>
                    <div class="bg-gray-100 p-6 rounded-lg shadow-lg">
                        <h3 class="text-xl font-bold mb-4">"Private Driver"</h3>
                        <p>"We offer professional private driver services..."</p>
                        <button class="mt-4 bg-green-500 text-white px-4 py-2 rounded">"Coming Soon"</button>
                    </div>
                    <div class="bg-gray-100 p-6 rounded-lg shadow-lg">
                        <h3 class="text-xl font-bold mb-4">"Long Trip"</h3>
                        <p>"Plan your long trips with us..."</p>
                        <button class="mt-4 bg-green-500 text-white px-4 py-2 rounded">"Coming Soon"</button>
                    </div>
                </div>
            </div>
        </section>
    }
}
