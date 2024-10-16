use crate::app;
use fuel_dao_leptos_csr::app:*;
use leptos::*;
fn main() {
    // set up logging
    _ = console_log::init_with_level(log::level::debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <app /> }
    })
}
