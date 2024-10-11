use chrono::NaiveDateTime;
use leptos::{expect_context, logging, RwSignal, SignalGetUntracked, SignalUpdate, Trigger};

use crate::canister::backend::CarDetails;

#[derive(Clone, Default)]
pub struct CheckoutState {
    pub selected_car: RwSignal<Option<CarDetails>>,
    pub start_time: RwSignal<Option<u64>>,
    pub end_time: RwSignal<Option<u64>>,
    pub pickup_date_formatted: RwSignal<String>, 
    pub return_date_formatted: RwSignal<String>,

}

#[derive(Clone, Default)]
pub struct CheckoutUser {
    pub user: RwSignal<UserDetials>,
    pub on_form_reset: Trigger,
}

#[derive(Clone, Default)]
pub struct UserDetials {
    pub name: Option<String>,
    pub email: Option<String>,
    pub country_code: Option<String>,
    pub mobile_number: Option<String>,
    pub age: Option<u8>,
    pub pan: Option<String>, 
    pub aadhar: Option<String,>
}

fn check_is_some_and_not_empty(val: &Option<String> ) -> bool {
    val.is_some() && val.as_ref().unwrap().len() > 0
}

impl UserDetials {
    pub fn check_ready(&self) -> bool {
        check_is_some_and_not_empty(&self.name)  &&  check_is_some_and_not_empty(&self.email) && check_is_some_and_not_empty(&self.country_code) && self.age.is_some() && check_is_some_and_not_empty(&self.pan) && check_is_some_and_not_empty(&self.aadhar)
    }

    
}

impl CheckoutState {
    pub fn get() -> Self {
        let this: Self = expect_context();
        this
    }

    pub fn set(car: CarDetails) {
        let this: Self = expect_context();

        this.selected_car.update(|f| *f = Some(car));
    }

    pub fn set_pickup_date_value(value: u64) {
        let this: Self = expect_context();

        this.start_time.update(|f| *f = Some(value));

        if !Self::valid_time() {
        
            this.start_time.update(|f| *f = None);
                
        }

    }

    pub fn set_return_date_value(value: u64) {
        let this: Self = expect_context();

        this.end_time.update(|f| *f = Some(value));

        if !Self::valid_time() {
        
        this.end_time.update(|f| *f = None);
            
        }
    }

    fn valid_time() -> bool {
        let this: Self = expect_context();

        match (this.start_time.get_untracked(), this.end_time.get_untracked()) {
            (Some(start), Some(end)) => start < end,
            _ => true,
        }
    }

    pub fn set_pickup_date_value_formatted(value: String) {
        let this: Self = expect_context();

        this.pickup_date_formatted.update(|f| *f = value.clone());

        let time = format!("{}", value.clone());
        match   NaiveDateTime::parse_from_str(&time, "%Y-%m-%dT%H:%M") {
            Ok(date) => {
                let datetime = date.and_utc().timestamp();
                Self::set_pickup_date_value(datetime as u64);
                }, 
            Err(e) =>  {
                logging::log!("failed to parse datetime {:?}", e);
            }
        };
    }

    pub fn set_return_date_value_formatted(value: String) {
        let this: Self = expect_context();

        this.return_date_formatted.update(|f| *f = value.clone());

        let time = format!("{}", value.clone());
        match   NaiveDateTime::parse_from_str(&time, "%Y-%m-%dT%H:%M") {
            Ok(date) => {
                let datetime = date.and_utc().timestamp();
                Self::set_return_date_value(datetime as u64);
                }, 
            Err(e) =>  {
                logging::log!("failed to parse datetime {:?}", e);
            }
        };
    }

    

    pub fn clear() {
        let this: Self = expect_context();
        this.selected_car.update(|f| *f = None);
    }
}
