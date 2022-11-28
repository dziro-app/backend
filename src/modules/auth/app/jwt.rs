use chrono::Utc;
use jsonwebtoken::{encode, decode, Header, Algorithm, EncodingKey, DecodingKey, Validation, errors::Error};


use crate::modules::auth::{dtos::Claim};


pub enum TokenType {
  Refresh,
  Access
}

pub struct JwtManager {
  secret: String,
}

impl JwtManager {
  pub fn new(secret: String) -> Self {
    return JwtManager { secret: secret }
  }

  pub fn create_jwt(&self, uid: String, token_type: TokenType) -> Result<String, Error> {
    let session_minutes = 5;
    let duration = match token_type {
      TokenType::Refresh => {
        chrono::Duration::minutes(session_minutes)
      },
      TokenType::Access => {
        chrono::Duration::minutes(session_minutes + 1)
      }
    };

    let expiration = Utc::now()
      .checked_add_signed(duration)
      .expect("valid timestamp")
      .timestamp();
  
    let claims = Claim {
      sub: uid,
      exp: expiration as usize
    };
  
    let header = Header::new(Algorithm::HS512);
  
    let f = encode(&header, &claims, &EncodingKey::from_secret(self.secret.as_bytes()))?;
  
    return Ok(f);
  }

  pub fn validate_jwt(&self, jwt: String) -> Result<Claim, Error> {
    let decoded = decode::<Claim>(
      &jwt,
      &DecodingKey::from_secret(self.secret.as_bytes()),
      &Validation::new(Algorithm::HS512)
    )?;
  
    Ok(decoded.claims)
  }
}



#[cfg(test)]
mod auth {
  use super::*;

  #[test]
  fn ut_test_jwt() {
    let uuid = String::from("e37f5ca9-fba1-46d9-a3ef-e80ada650784");
    let secret = String::from("supersecret");
    let manager = JwtManager::new(secret);

    let jwt = match manager.create_jwt(uuid.clone(), TokenType::Access) {
      Ok(j) => {j}, 
      Err(e) => {
        panic!("{}", e);
      }
    };

    match manager.validate_jwt(jwt.clone()) {
      Ok(claim) => {
        print!("{:#?}", claim);
        assert_eq!(uuid, claim.sub);
      }, 
      Err(e) => {
        panic!("{}", e);
      }
    };
  }
}