// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};
type Result<T> = std::result::Result<T, ic_agent::AgentError>;

#[derive(CandidType, Deserialize, serde::Serialize, Debug, Clone)]
pub enum CarStatus {
  OutOfService{ reason: String },
  Available,
  Reserved{
    reservation_id: Principal,
    reservation_timestamp: u64,
    customer_id: Principal,
  },
  UnderMaintenance,
  ComingSoon,
  ScheduledForInspection{ inspection_timestamp: u64 },
  Unavailable,
}

#[derive(CandidType, Deserialize, serde::Serialize, Debug, Clone)]
pub enum CarType { #[serde(rename="SUV")] Suv, Sedan, Truck, Coupe }

#[derive(CandidType, Deserialize, serde::Serialize, Debug, Clone)]
pub struct Location {
  pub latitude: f64,
  pub longitude: f64,
  pub address: String,
}

#[derive(CandidType, Deserialize, serde::Serialize, Debug, Clone)]
pub enum FuelType { Petrol, Electric, Hybrid, Diesel }

#[derive(CandidType, Deserialize, serde::Serialize, Debug, Clone)]
pub enum TransmissionType { Manual, Automatic }

#[derive(CandidType, Deserialize, serde::Serialize, Debug, Clone)]
pub struct CarDetails {
  pub id: u64,
  pub status: CarStatus,
  pub model: String,
  pub mileage: Option<u32>,
  pub make: String,
  pub color: Option<String>,
  pub year: u32,
  pub description: String,
  pub current_price_per_day: f64,
  pub default_image_url: String,
  pub car_type: CarType,
  pub pickup_location: Option<Location>,
  pub dropoff_location: Option<Location>,
  pub capacity: u8,
  pub fuel_type: FuelType,
  pub price_per_day: f64,
  pub transmission_type: TransmissionType,
}

#[derive(CandidType, Deserialize, serde::Serialize, Debug, Clone)]
pub enum Result_ { Ok(String), Err(String) }

#[derive(CandidType, Deserialize, serde::Serialize, Debug, Clone)]
pub struct CustomerDetials {
  pub age: u8,
  pub pan: String,
  pub mobile_number: String,
  pub name: String,
  pub email: String,
  pub country_code: String,
  pub aadhar: String,
}

#[derive(CandidType, Deserialize, serde::Serialize, Debug, Clone)]
pub enum PaymentStatus { Paid, Unpaid }

#[derive(CandidType, Deserialize, serde::Serialize, Debug, Clone)]
pub struct RentalTransaction {
  pub end_timestamp: u64,
  pub total_amount: f64,
  pub customer: Option<CustomerDetials>,
  pub start_timestamp: u64,
  pub customer_principal_id: Principal,
  pub payment_status: PaymentStatus,
  pub car_principal_id: u64,
}

#[derive(CandidType, Deserialize, serde::Serialize, Debug, Clone)]
pub struct CarAvailability {
  pub available: Option<RentalTransaction>,
  pub details: CarDetails,
}

#[derive(CandidType, Deserialize, serde::Serialize, Debug, Clone)]
pub struct Car {
  pub id: u64,
  pub bookings: Vec<RentalTransaction>,
  pub details: CarDetails,
}

#[derive(CandidType, Deserialize, serde::Serialize, Debug, Clone)]
pub enum EventMoniter {
  CarCheckout{ user_principal: Principal, car_id: u64, current_timestamp: u64 },
  SelectedCar{ user_principal: Principal, car_id: u64, current_timestamp: u64 },
  SearchInitiate{ user_principal: Principal, current_timestamp: u64 },
}

#[derive(CandidType, Deserialize, serde::Serialize, Debug, Clone)]
pub enum Result1 { Ok(RentalTransaction), Err(String) }

pub struct Backend<'a>(pub Principal, pub &'a ic_agent::Agent);
impl<'a> Backend<'a> {
  pub async fn add_car(&self, arg0: CarDetails) -> Result<()> {
    let args = Encode!(&arg0)?;
    let bytes = self.1.update(&self.0, "add_car").with_arg(args).call_and_wait().await?;
    Ok(Decode!(&bytes)?)
  }
  pub async fn cancel_reservation(&self, arg0: u64) -> Result<Result_> {
    let args = Encode!(&arg0)?;
    let bytes = self.1.update(&self.0, "cancel_reservation").with_arg(args).call_and_wait().await?;
    Ok(Decode!(&bytes, Result_)?)
  }
  pub async fn get_car_details(
    &self,
    arg0: u64,
    arg1: u64,
    arg2: u64,
  ) -> Result<Option<CarAvailability>> {
    let args = Encode!(&arg0, &arg1, &arg2)?;
    let bytes = self.1.update(&self.0, "get_car_details").with_arg(args).call_and_wait().await?;
    Ok(Decode!(&bytes, Option<CarAvailability>)?)
  }
  pub async fn get_default_car(&self) -> Result<Option<Car>> {
    let args = Encode!()?;
    let bytes = self.1.query(&self.0, "get_default_car").with_arg(args).call().await?;
    Ok(Decode!(&bytes, Option<Car>)?)
  }
  pub async fn get_monitoring_events(&self) -> Result<Vec<EventMoniter>> {
    let args = Encode!()?;
    let bytes = self.1.query(&self.0, "get_monitoring_events").with_arg(args).call().await?;
    Ok(Decode!(&bytes, Vec<EventMoniter>)?)
  }
  pub async fn greet(&self, arg0: String) -> Result<String> {
    let args = Encode!(&arg0)?;
    let bytes = self.1.query(&self.0, "greet").with_arg(args).call().await?;
    Ok(Decode!(&bytes, String)?)
  }
  pub async fn list_all_cars(&self) -> Result<Vec<Car>> {
    let args = Encode!()?;
    let bytes = self.1.update(&self.0, "list_all_cars").with_arg(args).call_and_wait().await?;
    Ok(Decode!(&bytes, Vec<Car>)?)
  }
  pub async fn reserve_car(
    &self,
    arg0: u64,
    arg1: u64,
    arg2: u64,
    arg3: CustomerDetials,
  ) -> Result<Result1> {
    let args = Encode!(&arg0, &arg1, &arg2, &arg3)?;
    let bytes = self.1.update(&self.0, "reserve_car").with_arg(args).call_and_wait().await?;
    Ok(Decode!(&bytes, Result1)?)
  }
  pub async fn search_car(&self, arg0: u64, arg1: u64) -> Result<Vec<Car>> {
    let args = Encode!(&arg0, &arg1)?;
    let bytes = self.1.query(&self.0, "search_car").with_arg(args).call().await?;
    Ok(Decode!(&bytes, Vec<Car>)?)
  }
  pub async fn update_car(&self, arg0: u64, arg1: CarDetails) -> Result<()> {
    let args = Encode!(&arg0, &arg1)?;
    let bytes = self.1.update(&self.0, "update_car").with_arg(args).call_and_wait().await?;
    Ok(Decode!(&bytes)?)
  }
}
