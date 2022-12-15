use serde_json;
use serde::{Serialize, Deserialize};
use rocket::serde::json::{Json};
use rocket::{get, post, State, http::{Status, Cookie, SameSite, CookieJar}, response::{content, status}};

use crate::infra::state::AppState;
use crate::modules::auth::{
  infra::spotify,
  app::jwt::{JwtManager, TokenType},
  dtos::{JwtResponse, UserResponse}
};
use crate::modules::users::{
  app::Manager as UserManager,
  dtos::CreateUser
};


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
      let config = state.settings.spotify.clone();

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
      
      let profile_image:String;
      let config = state.settings.spotify.clone();
      let token = spotify::get_auth_token(config, code_info.code.clone()).await.unwrap();
      let user_data = spotify::get_user_info(token).await.unwrap();
      
      let user_manager = UserManager {
        repo: Box::new(state.repositories.user.clone())
      };
      
      if user_data.images.len() == 0 {
        profile_image = "https://tresubresdobles.com/wp-content/uploads/2021/04/skft-23aff38e10ee3c4e430a1f3450c4a01d.jpeg".to_string();
      } else {
        profile_image = user_data.images[0].url.clone()
      }

      let user = match user_manager.find(user_data.id.clone()) {
        Ok(u) => {u},
        Err(_) => {
          let new_user = CreateUser {
            id: user_data.id,
            username: user_data.display_name
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

      let jwt_manager = JwtManager::new(state.settings.jwt.secret.clone(), state.settings.jwt.expiration.clone());

      let access_token = match jwt_manager.create_jwt(user.id.clone(), TokenType::Access) {
        Ok(c) => {c},
        Err(e) => { 
          println!("{}", e);
          return status::Custom(Status::InternalServerError, content::RawJson(String::from("{}"))); }
      };

      let refresh_token = match jwt_manager.create_jwt(user.id, TokenType::Refresh) {
        Ok(c) => {c},
        Err(e) => { 
          println!("{}", e);
          return status::Custom(Status::InternalServerError, content::RawJson(String::from("{}"))); }
      };

      let mut cookie = Cookie::build("refresh_token", refresh_token).finish();

      cookie.set_same_site(SameSite::None);

      cookies.add(cookie);

      let response = JwtResponse {
        access_token: access_token,
        user: UserResponse {
          username: user.username,
          profile: profile_image.clone()
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


#[post("/refresh")]
pub fn refresh_token(state: &State<AppState>, cookies: &CookieJar<'_>) -> status::Custom<content::RawJson<String>> {

  let refr = match cookies.get("refresh_token") {
    Some(t) => {t.value()},
    None => {
      return status::Custom(Status::BadRequest, content::RawJson(String::from("{}")));
    }
  };

  let jwt_manager = JwtManager::new(state.settings.jwt.secret.clone(), state.settings.jwt.expiration.clone());

  let claims = match jwt_manager.validate_jwt(refr.to_string()) {
    Ok(c) => {c},
    Err(e) => {
      println!("{}", e);
      return status::Custom(Status::BadRequest, content::RawJson(String::from("{}")));
    }
  };


  let access_token = match jwt_manager.create_jwt(claims.sub.clone(), TokenType::Access) {
    Ok(c) => {c},
    Err(e) => { 
      println!("{}", e);
      return status::Custom(Status::InternalServerError, content::RawJson(String::from("{}"))); }
  };

  let refresh_token = match jwt_manager.create_jwt(claims.sub, TokenType::Refresh) {
    Ok(c) => {c},
    Err(e) => { 
      println!("{}", e);
      return status::Custom(Status::InternalServerError, content::RawJson(String::from("{}"))); }
  };

  let mut cookie = Cookie::build("refresh_token", refresh_token).finish();

  cookie.set_same_site(SameSite::None);

  cookies.add(cookie);

  let content = format!("{{ \"access_token\": \"{}\" }}", access_token);

  return status::Custom(Status::Ok, content::RawJson(content));
} 

#[post("/logout")]
pub fn clear_token(cookies: &CookieJar<'_>) -> status::Custom<content::RawJson<String>> {
  let refr = match cookies.get("refresh_token") {
    Some(t) => {t},
    None => {
      return status::Custom(Status::BadRequest, content::RawJson(String::from("{}")));
    }
  };
  cookies.remove(refr.clone());
  return status::Custom(Status::Ok, content::RawJson("{}".to_string()));
}