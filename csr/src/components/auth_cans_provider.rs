
use leptos::*;

use crate::{
    state::canisters::{authenticated_canisters, Canisters},
    try_or_redirect_opt,
};

#[component]
pub fn AuthCansProvider<N, EF>(
    #[prop(into, optional)] fallback: ViewFn,
    children: EF,
) -> impl IntoView
where
    N: IntoView + 'static,
    EF: Fn(Canisters<true>) -> N + 'static + Clone,
{
    let cans_res = authenticated_canisters();
    let children = store_value(children);
    let loader = move || {
        let cans_wire = try_or_redirect_opt!((cans_res.0)()?);
        let cans = try_or_redirect_opt!(cans_wire.canisters());
        Some((children.get_value())(cans).into_view())
    };

    view! { <Suspense fallback=fallback>{loader}</Suspense> }
}
