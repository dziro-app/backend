use rocket::{get};

#[get("/<method>")]
pub fn get_third_token(method: String) -> String {
  format!("Token for {}", method)
} 