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
pub struct SpotifyConfig {
  pub client: String,
  pub secret: String,
  pub callback: String
}
#[derive(Debug, Deserialize)]
pub struct Settings {
  pub database: Database,
  pub jwt: JWT,
  pub spotify: SpotifyConfig
}

impl Settings {

  pub fn new() -> Result<Self, String> {
    dotenv().ok();
    
    let jwt_secret =  env::var("JWT_SECRET").expect("Missing jwt secret");
    let db_host = env::var("DB_HOST").expect("Missing db host");
    let db_name =  env::var("DB_NAME").expect("Missing db name");

    let spotify_client = env::var("SPOTIFY_CLIENT").expect("Missing spotify client");
    let spotify_secret = env::var("SPOTIFY_SECRET").expect("Missing spotify secret");
    let spotify_callback = env::var("SPOTIFY_CALLBACK").expect("Missing spotify callback");


    let jwt = JWT {
      secret: jwt_secret
    };

    let db = Database {
      name: db_name,
      host: db_host
    };

    let spotify_config = SpotifyConfig {
      client: spotify_client,
      secret: spotify_secret,
      callback: spotify_callback
    };

    return Ok(Settings {
      database: db,
      jwt: jwt,
      spotify: spotify_config
    })
  }
}