#[cfg(feature = "csr")]
pub fn main() {
    use fuel_dao_leptos_csr::app::App;
    use leptos::{get_configuration, logging::log};
    use leptos_router::Router;
    // Initialize logging (useful for client-side debugging)
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Use environment variables (these are usually embedded at build time in CSR)
    dotenv::dotenv().ok();

    // Get configuration (usually set during build time for CSR)
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;

    // Provide client-side route generation for navigation
    let routes = leptos_router::generate_route_list(App);

    // Mount the app to the body of the document (this is CSR specific)
    leptos::mount_to_body(move || {
        leptos::view! {
            <Router>
                <App />
            </Router>
        }
    });
}
