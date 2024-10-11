pub mod auth;
pub mod canisters;
pub mod checkout_state;
pub mod local_storage;

#[cfg(feature = "ssr")]
pub mod server {

    use crate::auth::server_impl::store::KVStoreImpl;

    use super::canisters::Canisters;
    use axum::extract::FromRef;
    use axum_extra::extract::cookie::Key;
    use leptos::LeptosOptions;
    use leptos_router::RouteListing;

    #[derive(FromRef, Clone)]
    pub struct AppState {
        pub leptos_options: LeptosOptions,
        pub canisters: Canisters<false>,
        pub kv: KVStoreImpl,
        pub routes: Vec<RouteListing>,
        pub cookie_key: Key,
    }
}
