use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
//use serde_json::Result;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[derive(Serialize, Deserialize)]
struct UserData {
  age: u8,
  driving_experience: u8,
  vehicle_make: String,
  vehicle_model: String,
  vehicle_year: u16,
}



#[wasm_bindgen]
pub fn get_insurance_quote(json_data: &str) {
  let user_data: Result<UserData, serde_json::Error> = serde_json::from_str(json_data);
  match user_data {
    Err(reason) => alert(&format!("Error: {}", reason))
    ,Ok(user_data) => {
      let quote_cents = calculate_quote(user_data);
      alert(&format!("Data: {}", quote_cents / 100.0 ));
    }
  }
}

fn calculate_quote(data: UserData) -> f64{
  // Fake database data
  match data {
    UserData { age, driving_experience, vehicle_make, vehicle_model, vehicle_year } 
      if vehicle_make == "Škoda" && vehicle_model == "Superb" && vehicle_year == 2015 => 32542f64 * age_and_experience_coeff(age.into()),

    UserData { age, driving_experience, vehicle_make, vehicle_model, vehicle_year } 
      if vehicle_make == "Ford" && vehicle_model == "Kuga" && vehicle_year == 2018 => 38549f64 * age_and_experience_coeff(age.into()),

    UserData { age, driving_experience, vehicle_make, vehicle_model, vehicle_year } 
      if vehicle_make == "BMW" && vehicle_model == "X7" && vehicle_year == 2019 => 58242f64 * age_and_experience_coeff(age.into()),

    UserData { age, driving_experience, vehicle_make, vehicle_model, vehicle_year } 
      if vehicle_make == "Daewoo" && vehicle_model == "Nubira" && vehicle_year == 2003 => 42544f64 * age_and_experience_coeff(age.into()),

    UserData { age, driving_experience, vehicle_make, vehicle_model, vehicle_year } 
      if vehicle_make == "Renault" && vehicle_model == "Zoe" && vehicle_year == 2019 => 21532f64 * age_and_experience_coeff(age.into()),

    UserData { age, driving_experience, vehicle_make, vehicle_model, vehicle_year } 
      if vehicle_make == "Opel" && vehicle_model == "Zafira" && vehicle_year == 2010 => 38751f64 * age_and_experience_coeff(age.into()),

    UserData { age, driving_experience, vehicle_make, vehicle_model, vehicle_year } 
      if vehicle_make == "Citroën" && vehicle_model == "C3" && vehicle_year == 2015 => 54532f64 * age_and_experience_coeff(age.into()),

    UserData {age, driving_experience, ..} => 40000f64 * age_and_experience_coeff(age.into())
  }
}

fn age_and_experience_coeff(age:f64) -> f64 {

  // y = 1/2312*(x-50)^2 + .5

  // 16 years -> 1.0
  // Decreases quadratically until 50 years, where it returns 0.5
  // 84 years -> 1.0
  ((1f64/2312f64)*(age - 50.0).powf(2.0)) as f64 + 0.5 
}
 
