use leptos::*;

#[component]
pub fn StayInTheKnow() -> impl IntoView {
    view! {
        <section class="bg-gray-900 text-white py-12">
            <div class="container mx-auto text-center">
                <h2 class="text-3xl font-bold mb-8">"Stay in the know"</h2>
                <div class="mb-8">
                    <p class="mb-4">"Sign up to get marketing emails from Bookme.com including promotions, rewards, travel experiences, and more."</p>
                    <div class="flex justify-center">
                        <input type="email" placeholder="Your email address" class="p-2 rounded-l-lg w-full max-w-xs text-black" />
                        <button class="bg-green-500 text-white px-4 py-2 rounded-r-lg">"Subscribe"</button>
                    </div>
                </div>
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-4">
                    { ["Pune", "Goa", "Port Blair", "Daman", "Delhi"].iter().map(|&place| {
                        view! {
                            <div class="bg-gray-800 rounded-lg overflow-hidden">
                                <img src={format!("path-to-{}.jpg", place.to_lowercase())} alt={place} class="w-full h-48 object-cover" />
                                <p class="text-center p-4">{place}</p>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}
