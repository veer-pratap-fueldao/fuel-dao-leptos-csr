use crate::components::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Main application component
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Content for your app's main structure and routes
    view! {
        <Stylesheet id="leptos" href="/pkg/fuel-dao-leptos-ssr.css" />
        <Title text="Welcome to Leptos" />

        // Defining the router and the routes for the application
        <Router>
            <main>
                <Routes>
                    <Route path="" view=BaseRoute>
                        <Route path="/" view=HomePage />
                        <Route path="/search" view=SearchResult />
                        <Route path="/checkout" view=CheckoutPage />
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

// Home page component
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Title text="FuelDao" />
        <main>
            <div class="flex flex-col min-h-screen">
                <Search />
                <BestPlacedForTrips />
                <Advantages />
                <InvestInCar />
                <GetInTouch />
                <Footer />
            </div>
        </main>
    }
}
