use serde_json;
use serde::{Serialize, Deserialize};
use rocket::serde::json::{Json};
use rocket::{get, post, State, http::{Status, Cookie, CookieJar}, response::{content, status}};

use crate::infra::state::AppState;
use crate::modules::auth::{
  infra::spotify,
  app::jwt::JwtManager,
  dtos::{JwtResponse, UserResponse}
};
use crate::modules::users::{
  app::Manager as UserManager,
  dtos::CreateUser
};

use super::spotify::SpotifyUserImage;



#[derive(Serialize)]
struct RedirectUrl {
  redirect: String
}

#[derive(Deserialize, Debug)]
pub struct CodeInfo {
  code: String
}


#[get("/<third_api>")]
pub fn get_third_token(third_api: String,  state: &State<AppState>) ->  status::Custom<content::RawJson<String>>  {
  let url = match third_api.as_str() {
    "spotify" => {
      let config = state.oauths.spotify.clone();

      spotify::get_token_url(config)
    },
    _ => {
      return status::Custom(Status::NotFound, content::RawJson(String::from("{}")));
    }
  };

  let l = RedirectUrl {
    redirect: url
  };
  let content = serde_json::to_string(&l).unwrap();
  return status::Custom(Status::Ok, content::RawJson(content));
}


#[post("/<third_api>/callback", format="application/json", data="<code_info>")]
pub async fn validate_third_token(third_api: String, code_info: Json<CodeInfo>, state: &State<AppState>, cookies: &CookieJar<'_>) ->  status::Custom<content::RawJson<String>>  {
 match third_api.as_str() {
    "spotify" => {
      let config = state.oauths.spotify.clone();
      let token = spotify::get_auth_token(config, code_info.code.clone()).await.unwrap();
      let mut user_data = spotify::get_user_info(token).await.unwrap();

      let user_manager = UserManager {
        repo: Box::new(state.repositories.user.clone())
      };

      if user_data.images.len() == 0 {
        let default_image = SpotifyUserImage {
          url: "https://tresubresdobles.com/wp-content/uploads/2021/04/skft-23aff38e10ee3c4e430a1f3450c4a01d.jpeg".to_string()
        };
        user_data.images.push(default_image);
      }

      let user = match user_manager.find(user_data.id.clone()) {
        Ok(u) => {u},
        Err(_) => {
          let new_user = CreateUser {
            id: user_data.id,
            username: user_data.display_name,
            profile_pic: user_data.images[0].url.clone()
          };
          match user_manager.create(new_user) {
            Ok(u) => {u},
            Err(e) => {
              println!("{}", e);
              return status::Custom(Status::InternalServerError, content::RawJson(String::from("{}")));
            }
          }
        }
      };

      let jwt_manager = JwtManager::new(state.jwt.secret.clone());

      let access_token = match jwt_manager.create_jwt(user.id.clone()) {
        Ok(c) => {c},
        Err(e) => { 
          println!("{}", e);
          return status::Custom(Status::InternalServerError, content::RawJson(String::from("{}"))); }
      };

      let refresh_token = match jwt_manager.create_jwt(user.id) {
        Ok(c) => {c},
        Err(e) => { 
          println!("{}", e);
          return status::Custom(Status::InternalServerError, content::RawJson(String::from("{}"))); }
      };

      cookies.add(Cookie::new("refresh_token", refresh_token));

      let response = JwtResponse {
        access_token: access_token,
        user: UserResponse {
          username: user.username,
          profile: user.profile_pic
        }
      };

      let content = serde_json::to_string(&response).unwrap();
      return status::Custom(Status::Ok, content::RawJson(content));
    },
    _ => {
      return status::Custom(Status::NotFound, content::RawJson(String::from("{}")));
    }
  };

}