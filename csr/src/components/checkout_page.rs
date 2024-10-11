use leptos::*;

use crate::{
    canister::backend::{CarAvailability, CarStatus, CustomerDetials, RentalTransaction}, components::{ActionTrackerPopup, Footer}, state::{
        canisters::authenticated_canisters,
        checkout_state::{CheckoutState, CheckoutUser},
    }, utils::{
        input::{non_empty_string_validator, InputBox},
        time::get_day_month_time,
    }
};

use super::Search;


#[component] 
pub fn CheckoutPage() -> impl IntoView {
    view! {
    <Search />
        <CheckoutPageInner />
    <Footer />
    }
 

}
#[component]
pub fn CheckoutPageInner() -> impl IntoView {
    let checkout = CheckoutState::get();

    match checkout.selected_car.get() {
        Some(car) => {
            let user: CheckoutUser = expect_context();
            let checkout_state: CheckoutState = expect_context();
            let cans_res = authenticated_canisters();
            let fetch_car = cans_res.clone();
            let fetch_car_details = create_resource(
                move || (checkout.start_time.get(), checkout.end_time.get()),
                move |_| {
                    let cans_res = fetch_car.clone();
                    async move {
                        let cans = cans_res.wait_untracked().await.unwrap();
                        let cans = cans.canisters().unwrap();
                        let car_details = cans
                            .backend()
                            .await
                            .get_car_details(
                                car.id,
                                checkout_state.start_time.get_untracked().unwrap(),
                                checkout.end_time.get_untracked().unwrap(),
                            )
                            .await;

                        match car_details {
                            Ok(cars) if cars.is_some() => Ok(cars.unwrap()),
                            _ => Err(format!("Some error occuered while fetching car details")),
                        }
                    }
                },
            );

            
            let create_action = create_action( move |&()| {
                {
                let value = cans_res.clone();
                async move {

                    if !user.user.get_untracked().check_ready() {
                        return Err("All fields are required".into());
                    }

                    let cans = value.wait_untracked().await.unwrap();
                        let cans = cans.canisters().unwrap();
                        let backend = cans
                            .backend()
                            .await;
                        let reserve  = backend.reserve_car(car.id, checkout.start_time.get_untracked().unwrap(), checkout.end_time.get_untracked().unwrap(), CustomerDetials {
                            age:   user.user.get_untracked().age.unwrap(), 
                            pan :  user.user.get_untracked().pan.unwrap(), 
                            aadhar:  user.user.get_untracked().aadhar.unwrap(), 
                            mobile_number:  user.user.get_untracked().mobile_number.unwrap(),
                            name:  user.user.get_untracked().name.unwrap(),
                            email:  user.user.get_untracked().email.unwrap(), 
                            country_code:  user.user.get_untracked().country_code.unwrap()
                        }).await;

                        match reserve {
                            Ok(result1)  => {
                                    match result1 {
                                        crate::canister::backend::Result1::Ok(rental_transaction) => Ok(rental_transaction),
                                        crate::canister::backend::Result1::Err(e) => Err(e),
                                    }
                            }  , 
                            Err(e) => Err(e.to_string())                           
                        }


                }
                }


            });

            let creating = create_action.pending();

            let create_diabled = create_memo(move |_| creating() || !user.user.get().check_ready() );

            let update_name = move |value| user.user.update(|f| f.name = Some(value));
            let update_age = move |value: String| {
                user.user
                    .update(|f| {
                        let age = value.parse::<u8>();
                        match age {
                            Ok(age) => {f.age = Some(age);}, 
                            Err(_) => {f.age = None}
                        }
                        })
            };
            let update_email = move |value| user.user.update(|f| f.email = Some(value));
            let update_code = move |value| user.user.update(|f| f.country_code = Some(value));
            let update_mobile = move |value| user.user.update(|f| f.mobile_number = Some(value));
            let update_pan = move |value| user.user.update(|f| f.pan = Some(value));
            let update_aadhar = move |value| user.user.update(|f| f.aadhar = Some(value));

            // let detals = car.clone();
            view! {
                <div class="container mx-auto py-8 px-4 lg:px-8">
                    <div class="bg-white rounded-lg  p-6 mb-6">
                    <h1 class="text-2xl lg:text-3xl font-bold mb-6">{format!("{} {}",car.make, car.model )}</h1>
                    </div>
                    
                    <Suspense>
                    {
                        move || {
                            fetch_car_details.get().map(|res| {
                                match res {
                                    Ok(avail) => {
                                        /* view! {
                                            <div>
                                            <CheckoutInner avail />
                                            </div>
                                        } */
                                       let t_n_c =   create_rw_signal(false);

                                        let car = avail.details;
                                        let available = avail.available;
                                        let status = car.status.clone();
                                        view! {
                    <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                        // <!-- Left Section - Billing Info, Payment, and Confirmation -->
                        <div class="lg:col-span-2">
                            // <!-- Billing Info -->
                            <div class="bg-white rounded-lg shadow-lg p-6 mb-6">
                                <div class="flex justify-between items-center mb-4">
                                    <h2 class="text-xl font-bold">"Billing Info"</h2>
                                    <span class="text-sm text-gray-500">"Step 1 of 2"</span>
                                </div>
                                <p class="text-sm text-gray-400 mb-4">"Please enter your billing info"</p>

                                <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
                                    // <input type="text" placeholder="Full Name" class="border rounded-lg px-4 py-2"/>
                                    <InputBox initial_value=user.user.get_untracked().name.map_or("".into(), |f| f ) heading="Full Name"  placeholder="Full Name" updater=update_name validator=move|val|non_empty_string_validator(val) />
                                    // <input type="text" placeholder="Age" class="border rounded-lg px-4 py-2"/>
                                    <InputBox initial_value=user.user.get_untracked().age.map_or("".into(), |f| f.to_string() ) heading="Age" placeholder="Age" updater=update_age validator=move|val|non_empty_string_validator(val) input_type="number" />
                                    // <input type="email" placeholder="Email ID" class="border rounded-lg px-4 py-2"/>
                                    <InputBox initial_value=user.user.get_untracked().email.map_or("".into(), |f| f.to_string() ) heading="Email" placeholder="Email" updater=update_email validator=move|val|non_empty_string_validator(val) input_type="email" />
                                   
                                    // <input type="text" placeholder="Pan Card No." class="border rounded-lg px-4 py-2"/>
                                    <InputBox initial_value=user.user.get_untracked().pan.map_or("".into(), |f| f.to_string() ) heading="PAN" placeholder="PAN" updater=update_pan validator=move|val|non_empty_string_validator(val) />
                                    
                                    // <input type="text" placeholder="Adhar Card No." class="border rounded-lg px-4 py-2"/>
                                    <InputBox initial_value=user.user.get_untracked().aadhar.map_or("".into(), |f| f.to_string() ) heading="Aadhar" placeholder="Aadhar" updater=update_aadhar validator=move|val|non_empty_string_validator(val) input_type="number" />

                                </div>
                                <div class="grid grid-cols-2 space-x-2 gap-4">
                                    // <input type="text" placeholder="Country Code" class="border rounded-lg px-4 py-2 w-1/3"/>
                                    <InputBox initial_value=user.user.get_untracked().country_code.map_or("".into(), |f| f.to_string() ) heading="Country Code" placeholder="Country Code" updater=update_code validator=move|val|non_empty_string_validator(val) input_type="text" style="col-span-1".into() />
                                    // <input type="text" placeholder="Mobile Number" class="border rounded-lg px-4 py-2 w-2/3"/>
                                    <InputBox initial_value=user.user.get_untracked().mobile_number.map_or("".into(), |f| f.to_string() ) heading="Mobile Number" placeholder="Mobile Number" updater=update_mobile validator=move|val|non_empty_string_validator(val) input_type="number" style="col-span-2".into() />
                                </div>
                            </div>

                            // <!-- Payment Method -->
                            // <PaymentMethod />

                            // <!-- Confirmation -->
                            <div class="bg-white rounded-lg shadow-lg p-6 mb-6">
                                <div class="flex justify-between items-center mb-4">
                                    <h2 class="text-xl font-bold">"Confirmation"</h2>
                                    <span class="text-sm text-gray-500">"Step 2 of 2"</span>
                                </div>
                                <p class="text-sm text-gray-400 mb-4">"We are getting to the end. Just a few clicks and your rental is ready!"</p>

                                <div class="flex items-center mb-6">
                                    <input type="checkbox" id="terms" class="mr-2" on:input=move|_| {t_n_c.update(|f| *f = !*f);}/>
                                    <label for="terms" class="text-gray-600 text-sm">"I agree with our terms and conditions and privacy policy."</label>
                                </div>
                                <Show when=move||(format!("{:?}", car.status) == format!("{:?}", CarStatus::Available ) && t_n_c.get() && user.user.get().check_ready())   fallback=move || view! {
                                    <button disabled=true class="w-full bg-gray-500 text-white py-3 rounded-lg font-bold">
                                      { match  format!("{:?}", status) == format!("{:?}", CarStatus::Available) {
                                        true => "Rent Now", 
                                        false => "Not Available"
                                      } }
                                    </button>
                                }>
                                <button disabled=create_diabled  on:click=move |_| create_action.dispatch(()) class="w-full bg-green-500 text-white py-3 rounded-lg font-bold disabled:text-neutral-500 disabled:bg-primary-500/30">
                                    "Rent Now"
                                </button>
                                <BookingCreationPopup creation_action=create_action/>
                                </Show>
                            </div>

                            // <!-- Security Info -->
                            <div class="flex items-center text-gray-600 text-sm">
                                <img src="/icons/security.svg" class="h-6 w-6 mr-2 text-blue-500"  />

                                <span>"All your data is safe. We are using the most advanced security to provide you the best experience ever."</span>
                            </div>
                        </div>

                        // <!-- Right Section - Rental Summary -->
                        
                                                <div class="bg-white rounded-lg shadow-lg p-6">
                            <h2 class="text-xl font-bold mb-4">"Rental Summary"</h2>

                            <div class="flex justify-between items-center mb-4">
                                <img src=car.default_image_url alt="Car image" class="w-20 h-16 rounded-lg"/>
                                <div>
                                    <h3 class="text-lg font-bold">{format!("{} {}", car.make, car.model.clone())}</h3>
                                    // <p class="text-sm text-gray-400">"440+ Reviewer"</p>
                                </div>
                            </div>

                            <div class="text-sm text-gray-600 mb-4">
                                {
                                    match car.pickup_location {
                                        Some(location) =>{ view! { <div class="flex justify-between items-center mb-2">
                                                                        <span>"Pick up Location"</span>
                                                                        <span>{ location.address}</span>
                                                                    </div> } }
                                    None => {view! { <div> </div> }}
                                    }
                                }

                                <div class="flex justify-between items-center mb-2">
                                    <span>"Date & Time"</span>
                                    <span>{move|| get_day_month_time(checkout.start_time.get().unwrap())}</span>
                                </div>
                                {
                                    match car.dropoff_location {
                                        Some(location) =>{ view! { <div class="flex justify-between items-center mb-2">
                                                                        <span>"Pick up Location"</span>
                                                                        <span>{ location.address}</span>
                                                                    </div> } }
                                    None => {view! { <div> </div> }}
                                    }
                                }
                                // <div class="flex justify-between items-center mb-2">
                                //     <span>"Drop off Location"</span>
                                //     <span>"Same Location"</span>
                                // </div>
                                <div class="flex justify-between items-center mb-2">
                                    <span>"Date & Time"</span>
                                    <span>{move || get_day_month_time(checkout.end_time.get().unwrap())}</span>
                                </div>
                            </div>

                            {
                                match available {
                                    Some(a) => {
                                        view! {
                                        <div>
                                            // {a.customer_principal_id.to_text()}
                                            <div class="flex justify-between items-center mb-4">
                                            <span>"Subtotal"</span>
                                            <span>"₹"{format!("{:.2}", a.total_amount)}</span>
                                        </div>
                                        // <div class="flex justify-between items-center mb-4">
                                        //     <span>"Tax"</span>
                                        //     <span>"₹""0.00"</span>
                                        // </div>

                                        // <div class="flex items-center mb-4">
                                        //     <input type="text" placeholder="Promo Code" class="border rounded-lg px-4 py-2 flex-1"/>
                                        //     <button class="bg-gray-200 text-gray-600 py-2 px-4 rounded-lg ml-2">"Apply"</button>
                                        // </div>

                                        <div class="flex justify-between items-center mb-4 font-bold text-lg">
                                            <span>"Total Rental Price"</span>
                                            <span>"₹"{format!("{:.2}", a.total_amount)}</span>
                                        </div>

                                        // <button class="w-full bg-green-500 text-white py-3 rounded-lg font-bold hover:bg-green-600">
                                        //     "Apply Now"
                                        // </button>
                                            </div>
                                        }

                                    },
                                    None => view! {<div> </div>}
                                }
                            }


                            </div>
                                            

                            </div>
                                }
                                },
                                Err(_) => {
                                    view! {
                                        <div></div>
                                    }
                                    } ,
                                    }


                                 })
                             }
                    }
                    </Suspense>
                </div>
            }
        }
        None => {
            view! {
                <div class="bg-white rounded-lg shadow-lg p-6 mb-6">
                <h1 class="text-2xl lg:text-3xl font-bold mb-6">Select a car to proceed</h1>
                </div>
            }
        }
    }
}


#[component]
pub fn BookingCreationPopup(
    creation_action: Action<(), Result<RentalTransaction, String>>,
) -> impl IntoView {
    let close_popup = create_rw_signal(false);
    view! {
        <ActionTrackerPopup
            action=creation_action
            loading_message="Booking in Progress"
            modal=move |res| match res {
                Ok(_) => {
                    view! {
                        <div>
                        <div class="flex items-center justify-center  bg-gray-100">
            <div class="bg-white p-2 rounded-lg shadow-lg text-center max-w-md w-full">
                // Success Icon
                <div class="flex justify-center">
                <svg xmlns="http://www.w3.org/2000/svg" width="72" height="72" fill="green" viewBox="0 0 24 24">
                <path d="M12 0C5.383 0 0 5.383 0 12c0 6.616 5.383 12 12 12 6.616 0 12-5.384 12-12 0-6.617-5.384-12-12-12zm-1.015 17.828l-4.813-4.828 1.427-1.413 3.4 3.413 7.399-7.428 1.414 1.399-8.827 8.857z"/>
                </svg>
                </div>
                
                // Success Message
                <h2 class="text-2xl font-semibold text-gray-800 mb-2">
                    "We've sent you a booking confirmation on your email."
                </h2>
                <p class="text-gray-600 mb-2">
                    "We'll communicate further instructions on the same email"
                </p>
                
                // Booking Reference
                                        <div class="bg-gray-200 py-3 px-4 rounded-lg inline-block text-lg font-medium text-gray-700">
                                        "Booking Ref. FRA-BE-19283102-MHAKSA-DELIVERY-ORDER"
                                </div>
                            </div>
                        </div>
                        <div class="flex justify-center p-2">
                        <a  href="/search"  class="w-full  text-center bg-green-500 text-white px-3 py-3 rounded-lg font-bold disabled:text-neutral-500 disabled:bg-primary-500/30">
                        Back to Search
                        </a>
                        </div>
                        </div>
                    }
                }
                Err(e) => {
                    view! {
                        <div class="flex justify-center">
                        <p style="color:red">{e}</p>
                        <div class="flex justify-center">
                        <button   on:click=move |_|close_popup.set_untracked(false) class="w-full bg-green-500 text-white px-3 py-3 rounded-lg font-bold disabled:text-neutral-500 disabled:bg-primary-500/30">
                        "Retry"
                        </button>
                        </div>
                        
                        </div>
                    }
                }
            }

            close=close_popup
        />
    }
}


#[component]
fn PaymentMethod() -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-lg p-6 mb-6">
                                <div class="flex justify-between items-center mb-4">
                                    <h2 class="text-xl font-bold">"Payment Method"</h2>
                                    <span class="text-sm text-gray-500">"Step 3 of 4"</span>
                                </div>

                                <div class="mb-4">
                                    <input type="radio" id="creditCard" name="payment" checked class="mr-2"/>
                                    <label for="creditCard" class="text-lg font-medium">"Credit Card"</label>
                                </div>
                                <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 mb-4">
                                    <input type="text" placeholder="Card Number" class="border rounded-lg px-4 py-2"/>
                                    <input type="text" placeholder="Expiration Date (MM/YY)" class="border rounded-lg px-4 py-2"/>
                                    <input type="text" placeholder="Card Holder" class="border rounded-lg px-4 py-2"/>
                                    <input type="text" placeholder="CVC" class="border rounded-lg px-4 py-2"/>
                                </div>

                                <div class="mb-4">
                                    <input type="radio" id="paypal" name="payment" class="mr-2"/>
                                    <label for="paypal" class="text-lg font-medium">"PayPal"</label>
                                </div>

                                <div>
                                    <input type="radio" id="bitcoin" name="payment" class="mr-2"/>
                                    <label for="bitcoin" class="text-lg font-medium">"Bitcoin"</label>
                                </div>
        </div>
    }
}
