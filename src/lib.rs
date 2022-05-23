use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct UserData {
  age: u8,
  driving_experience: u8,
  vehicle_make: String,
  vehicle_model: String,
  vehicle_year: u16,
}

#[wasm_bindgen]
pub fn get_insurance_quote(json_data: &str) -> String {
  let user_data: Result<UserData, serde_json::Error> = serde_json::from_str(json_data);
  match user_data {
    Err(reason) => {
      format!("{{\"ok\": false, \"err_reason\": {} }}", reason)
    }
    ,Ok(user_data) => {
      let quote_cents = (calculate_quote(user_data) + 0.5) as i32;
      format!("{{\"ok\": true, \"quote_cents\":\"{}\" }}", quote_cents)
    }
  }
}

fn calculate_quote(data: UserData) -> f64{
  // Fake database data
  match data {
    UserData { age, driving_experience, vehicle_make, vehicle_model, vehicle_year } 
      if vehicle_make == "Škoda" && vehicle_model == "Superb" && vehicle_year == 2015 => 32542.0 * age_and_experience_coeff(age.into(), driving_experience.into()),

    UserData { age, driving_experience, vehicle_make, vehicle_model, vehicle_year } 
      if vehicle_make == "Ford" && vehicle_model == "Kuga" && vehicle_year == 2018 => 38549.0 * age_and_experience_coeff(age.into(), driving_experience.into()),

    UserData { age, driving_experience, vehicle_make, vehicle_model, vehicle_year } 
      if vehicle_make == "BMW" && vehicle_model == "X7" && vehicle_year == 2019 => 58242.0 * age_and_experience_coeff(age.into(), driving_experience.into()),

    UserData { age, driving_experience, vehicle_make, vehicle_model, vehicle_year } 
      if vehicle_make == "Daewoo" && vehicle_model == "Nubira" && vehicle_year == 2003 => 42544.0 * age_and_experience_coeff(age.into(), driving_experience.into()),

    UserData { age, driving_experience, vehicle_make, vehicle_model, vehicle_year } 
      if vehicle_make == "Renault" && vehicle_model == "Zoe" && vehicle_year == 2019 => 21532.0 * age_and_experience_coeff(age.into(), driving_experience.into()),

    UserData { age, driving_experience, vehicle_make, vehicle_model, vehicle_year } 
      if vehicle_make == "Opel" && vehicle_model == "Zafira" && vehicle_year == 2010 => 38751.0 * age_and_experience_coeff(age.into(), driving_experience.into()),

    UserData { age, driving_experience, vehicle_make, vehicle_model, vehicle_year } 
      if vehicle_make == "Citroën" && vehicle_model == "C3" && vehicle_year == 2015 => 54532.0 * age_and_experience_coeff(age.into(), driving_experience.into()),

    UserData {age, driving_experience, ..} => 40000.0 * age_and_experience_coeff(age.into(), driving_experience.into())
  }
}

fn age_and_experience_coeff(age:f64, exp:f64) -> f64 {

  // y = 1/2312*(x-50)^2 + .5

  // 16 years -> 1.0
  // Decreases quadratically until 50 years, where it returns 0.5
  // 84 years -> 1.0
  let age_coeff = if age >= 16.0 && age <= 84.0 {
                    ((1f64/2312f64)*(age - 50.0).powf(2.0)) as f64 + 0.5
                  }
                  else{
                    1.0
                  };

  let exp_coeff = if exp <= 20.0 && exp >= 0.0 {
                   0.5*(1.0 - exp/20.0) + 0.5
                   }
                   else if exp > 20.0{
                     0.5
                   }
                   else{
                     1.0
                   };
  age_coeff * exp_coeff
}
 
