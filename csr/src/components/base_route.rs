use candid::Principal;
use ic_agent::identity::Secp256k1Identity;
use k256::elliptic_curve::JwkEcKey;
use leptos::*;
use leptos_router::*;

use crate::consts::USER_PRINCIPAL_ID_STORE;
use crate::state::canisters::do_canister_auth;
use crate::utils::ParentResource;
use crate::{
    auth::{
        extract_identity, generate_anonymous_identity_if_required, set_anonymous_identity_cookie,
        DelegatedIdentityWire,
    },
    components::spinners::FullScreenSpinner,
    state::{
        auth::AuthState,
        canisters::{AuthCansResource, Canisters},
    },
    try_or_redirect,
    utils::MockPartialEq,
};
use codee::string::JsonSerdeCodec;
use leptos_use::storage::use_local_storage;

#[derive(Params, PartialEq, Clone)]
struct Referrer {
    user_refer: String,
}

#[component]
fn CtxProvider(temp_identity: Option<JwkEcKey>, children: ChildrenFn) -> impl IntoView {
    let auth = AuthState::default();
    provide_context(auth);

    let canisters_store = create_rw_signal(None::<Canisters<true>>);
    provide_context(canisters_store);

    let temp_identity_c = temp_identity.clone();
    create_local_resource(
        || (),
        move |_| {
            let temp_identity = temp_identity_c.clone();
            async move {
                let Some(id) = temp_identity else {
                    return;
                };
                if let Err(e) = set_anonymous_identity_cookie(id).await {
                    leptos::logging::log!("Failed to set anonymous identity as cookie?! err {e}");
                }
            }
        },
    );

    let canisters_res: AuthCansResource = ParentResource(create_resource(
        move || MockPartialEq(auth()),
        move |auth_id| {
            let temp_identity = temp_identity.clone();
            async move {
                if let Some(id_wire) = auth_id.0 {
                    return do_canister_auth(id_wire).await;
                }

                let Some(jwk_key) = temp_identity else {
                   let result: Result<_, SomeErrorType> = if let Some(id_wire) = extract_identity().await {
        Ok(id_wire)
    } else {
        Err(SomeErrorType::new("No refresh cookie set?!"))
    };

    result
                };

                let key = k256::SecretKey::from_jwk(&jwk_key)?;
                let id = Secp256k1Identity::from_private_key(key);
                let id_wire = DelegatedIdentityWire::delegate(&id);

                do_canister_auth(id_wire).await
            }
        },
    ));
    provide_context(canisters_res.clone());

    view! {
        {children}
        <Suspense>
            {move || {
                (canisters_res.0)()
                    .map(|res| {
                        let cans_wire = try_or_redirect!(res);
                        let cans = try_or_redirect!(cans_wire.canisters());
                        let (_, set_user_principal_id, _) = use_local_storage::<
                            Option<Principal>,
                            JsonSerdeCodec,
                        >(USER_PRINCIPAL_ID_STORE);
                        set_user_principal_id(Some(cans.user_principal()));
                        canisters_store.set(Some(cans));
                    })
            }}

        </Suspense>
    }
}

#[component]
pub fn BaseRoute() -> impl IntoView {
    let temp_identity_res = create_blocking_resource(
        || (),
        |_| async move {
            generate_anonymous_identity_if_required()
                .await
                .expect("Failed to generate anonymous identity?!")
        },
    );

    view! {
        <Suspense fallback=FullScreenSpinner>
            {move || {
                temp_identity_res()
                    .map(|temp_identity| {
                        view! {
                            <CtxProvider temp_identity>
                                <Outlet />
                            </CtxProvider>
                        }
                    })
            }}

        </Suspense>
    }
}
