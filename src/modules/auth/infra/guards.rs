use rocket::http::Status;
use rocket::request::{FromRequest, Request, Outcome};

use super::super::app::jwt::JwtManager;
use crate::infra::state::AppState;

pub struct AuthenticatedUser {
  pub id: String
}

#[derive(Debug)]
pub enum LoginError {
  Missing,
  InvalidData
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
  type Error = LoginError;

  async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let state = req.rocket().state::<AppState>().unwrap();
    let manager = JwtManager::new(String::from(state.jwt.secret.clone()));

    match req.headers().get_one("Authorization") {
      None => Outcome::Failure((Status::Unauthorized, LoginError::Missing)),
      Some(bearer) => {
        let b: Vec<&str> = bearer.split("Bearer ").collect();
        if b.len() != 2 {
          return Outcome::Failure((Status::BadRequest, LoginError::InvalidData));
        }
        let claims = match manager.validate_jwt(b[1].to_string()) {
          Ok(c) => c,
          Err(e) => {
            println!("{}", e);
            return Outcome::Failure((Status::BadRequest, LoginError::InvalidData));
          }
        };

        return Outcome::Success(AuthenticatedUser { id: claims.sub })
       },
    }
  }
}