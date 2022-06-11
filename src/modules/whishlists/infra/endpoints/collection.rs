use serde_json;
use rocket::serde::json::{Json};
use rocket::{get, post, patch, delete, State, http::Status, response::{content, status}};

use crate::infra::state::AppState;
use crate::modules::whishlists::app::collection;
use crate::modules::whishlists::dtos::collection::{CreateCollection, UpdateCollection};

#[get("/")]
pub fn get_collections(state: &State<AppState>) -> status::Custom<content::RawJson<String>> {
  let manager  = collection::Manager{
    repo: Box::new(state.repositories.collection.clone())
  };

  match manager.list() {
    Ok(l) => {
      let content = serde_json::to_string(&l).unwrap();
      return status::Custom(Status::Ok, content::RawJson(content));
    },
    Err(_e) => { 
      return status::Custom(Status::NotFound, content::RawJson(String::from("{}")));
    }
  }
}

#[post("/", format="application/json", data="<create>")]
pub fn create_collection(state: &State<AppState>, create: Json<CreateCollection>) -> status::Custom<content::RawJson<String>> {
  let manager  = collection::Manager{
    repo: Box::new(state.repositories.collection.clone())
  };

  match manager.create(CreateCollection { name: create.name.clone(), color: create.color.clone(), emoji: create.emoji.clone()}) {
    Ok(l) => {
      let content = serde_json::to_string(&l).unwrap();
      return status::Custom(Status::Created, content::RawJson(content));
    },
    Err(_e) => { 
      return status::Custom(Status::NotFound, content::RawJson(String::from("{}")));
    }
  }
}

#[patch("/<id>", format="application/json", data="<partial>")]
pub fn update_collection(state: &State<AppState>, id: String, partial: Json<UpdateCollection>) -> status::Custom<content::RawJson<String>> {

  let manager  = collection::Manager{
    repo: Box::new(state.repositories.collection.clone())
  };

  let data = UpdateCollection {
    name: partial.name.clone(),
    color: partial.color.clone(),
    emoji: partial.emoji.clone()
  };


  match manager.update(id, data) {
    Ok(l) => {
      let content = serde_json::to_string(&l).unwrap();
      return status::Custom(Status::Ok, content::RawJson(content));
    },
    Err(_e) => { 
      return status::Custom(Status::NotFound, content::RawJson(String::from("{}")));
    }
  }
}

#[delete("/<id>")]
pub fn delete_collection(state: &State<AppState>, id: String) -> status::Custom<content::RawJson<String>> {

  let manager  = collection::Manager{
    repo: Box::new(state.repositories.collection.clone())
  };

  match manager.delete(id) {
    Ok(_) => {
      return status::Custom(Status::Ok, content::RawJson(String::from("{}")));
    },
    Err(_e) => { 
      return status::Custom(Status::NotFound, content::RawJson(String::from("{}")));
    }
  }
}