pub mod app;
// pub mod auth;
pub mod canister;
pub mod components;
pub mod consts;
pub mod error_template;
pub mod state;
pub mod utils;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();

    // For client-side rendering (CSR), we only need to mount the app to the body of the DOM.
    leptos::mount_to_body(App);
}
