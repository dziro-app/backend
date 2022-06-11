use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Claim {
  pub sub: String,
  pub exp: usize
}

// Struct for successfull auth response
#[derive(Deserialize, Serialize, Debug)]
pub struct JwtResponse {
  pub access_token: String
}
