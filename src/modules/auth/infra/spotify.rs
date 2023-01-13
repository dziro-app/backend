use base64::{encode};
use reqwest::{self, StatusCode};
use serde::Deserialize;

use crate::modules::auth::infra::errors::{OauthResult, OauthError};



#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyAuthConfig {
  pub client: String,
  pub secret: String,
  pub callback: String,
  pub state: String
}

#[derive(Deserialize, Debug)]
pub struct SpotifyToken {
  pub access_token: String,
  pub expires_in: u16,
  pub token_type: String,
  pub refresh_token : String
}

#[derive(Deserialize, Debug)]
pub struct SpotifyUserImage { 
  pub url: String
}

#[derive(Deserialize, Debug)]
pub struct SpotifyUser {
  pub id: String,
  pub display_name: String,
  pub images: Vec<SpotifyUserImage>
}


pub fn get_token_url(config: SpotifyAuthConfig) -> String {
  let scopes = vec!["user-read-private".to_string()];
  
  let mut url =  String::from("https://accounts.spotify.com/authorize?");
  let response_type = String::from("response_type=code&");
  let client_id = format!("client_id={}&", config.client);
  let redirect_uri = format!("redirect_uri={}&", config.callback.clone());
  let state = format!("state={}&", config.state.clone());
  let scopes = format!("scopes={}", String::from(scopes.join(" ")));
  
  url.push_str(&client_id);
  url.push_str(&response_type);
  url.push_str(&redirect_uri);
  url.push_str(&state);
  url.push_str(&scopes);

  return url;
}

pub async fn get_auth_token(config: SpotifyAuthConfig, code: String) -> OauthResult<SpotifyToken> {
  let credentials = encode(format!("{}:{}", config.client, config.secret));

  let params = [
    ("code", code),
    ("redirect_uri", config.callback),
    ("grant_type", "authorization_code".to_string())
  ];

  let client = reqwest::Client::new();

  let response = match client.post("https://accounts.spotify.com/api/token")
    .form(&params)
    .header(reqwest::header::AUTHORIZATION, format!("Basic {}", credentials))
    .send()
    .await {
      Ok(r) => { 
        if r.status() != StatusCode::OK {
          return Err(OauthError::Authorization("Bad code".to_string()));
        } else { r }
      }
      Err(e) => {
        return Err(OauthError::Network(format!("{}", e)))
      }
    };

  match response.json::<SpotifyToken>().await {
    Ok(r) => { return Ok(r) },
    Err(e) => {
      println!("Error: {}", e);
      return Err(OauthError::MismatchingResponse(format!("{}", e)))
    }
  };

}

pub async fn get_user_info(token: SpotifyToken) -> OauthResult<SpotifyUser> {
  let client = reqwest::Client::new();
  let response = match client.get("https://api.spotify.com/v1/me")
    .header("Authorization", format!("{} {}", token.token_type, token.access_token))
    .send()
    .await {
      Ok(r) => { 
        if r.status() != StatusCode::OK {
          return Err(OauthError::Authorization("Bad token".to_string()));
        } else { r }
      }
      Err(e) => {
        return Err(OauthError::Network(format!("{}", e)))
      }
    };

  match response.json::<SpotifyUser>().await {
    Ok(r) => { return Ok(r) },
    Err(e) => {
      println!("Error: {}", e);
      return Err(OauthError::MismatchingResponse(format!("{}", e)))
    }
  };
}
