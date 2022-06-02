use std::env;

use dotenv::dotenv;
use serde::{Deserialize};


#[derive(Debug, Deserialize)]
pub struct JWT {
  pub secret: String
}

#[derive(Debug, Deserialize)]
pub struct Database {
  pub name: String,
  pub host: String
}

#[derive(Debug, Deserialize)]
pub struct Settings {
  pub database: Database,
  pub jwt: JWT
}

impl Settings {

  pub fn new() -> Result<Self, String> {
    dotenv().ok();
    let jwt_secret = match env::var("JWT_SECRET") {
      Ok(v) => { v },
      Err(e) => { panic!("Missing env var: {}", e)}
    };

    let db_host = match env::var("DB_HOST") {
      Ok(v) => { v },
      Err(e) => { panic!("Missing env var: {}", e)}
    }; 

    let db_name = match env::var("DB_NAME") {
      Ok(v) => { v },
      Err(e) => { panic!("Missing env var: {}", e)}
    }; 

    let db = Database {
      name: db_name,
      host: db_host
    };

    let jwt = JWT {
      secret: jwt_secret
    };

    return Ok(Settings {
      database: db,
      jwt: jwt
    })
  }
}