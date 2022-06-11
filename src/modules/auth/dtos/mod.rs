use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Claim {
  pub sub: String,
  pub exp: usize
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserResponse {
  pub username: String,
  pub profile: String
}
// Struct for successfull auth response
#[derive(Deserialize, Serialize, Debug)]
pub struct JwtResponse {
  pub access_token: String,
  pub user: UserResponse
}
