use reqwest::{self, StatusCode};
use serde::Deserialize;
use base64;

use crate::modules::auth::infra::errors::{OauthResult, OauthError};

#[derive(Deserialize)]
pub struct FacebookImage {
  pub url: String
}

#[derive(Deserialize)]
pub struct FacebookImageData {
  pub data:FacebookImage
}

#[derive(Deserialize)]
pub struct FacebookResponse {
  pub id: String,
  pub name: String,
  pub picture: FacebookImageData
}

#[derive(Deserialize)]
pub struct FacebookUser {
  pub id: String,
  pub name: String,
  pub picture: String
}


pub async fn get_user_info(token: String) -> OauthResult<FacebookUser> {
  let client = reqwest::Client::new();
  let url = format!("https://graph.facebook.com/v15.0/me?fields=id%2Cname%2Cpicture&access_token={}", token);
  let response = match client.get(url).send().await {
    Ok(r) => {
      if r.status() != StatusCode::OK {
        return Err(OauthError::Authorization(token))
      } else { r }
    }
    Err(e) => {
      return Err(OauthError::Network(format!("{}", e)))
    }
  };

  let user_response = match response.json::<FacebookResponse>().await {
    Ok(fb_u) => { fb_u },
    Err(e) => {
      return Err(OauthError::MismatchingResponse(format!("{}", e)))
    }
  };

  let image_response = match client.get(user_response.picture.data.url).send().await{
    Ok(r) => {r},
    Err(e) => {
      return Err(OauthError::Network(format!("{}", e)))
    }
  };

  match image_response.bytes().await {
    Ok(raw) => {
      let image = base64::encode(&raw);
      Ok(FacebookUser { id: user_response.id, name: user_response.name, picture: format!("data:image/png;base64,{}", image) })
    }
    Err(e) => {
      Err(OauthError::MismatchingResponse(format!("bad image: {}", e)))
    }
  }

}