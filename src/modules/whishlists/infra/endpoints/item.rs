use serde_json;
use rocket::serde::json::{Json};
use rocket::{post, patch, delete, State, http::Status, response::{content, status}};

use crate::infra::state::AppState;
use crate::modules::whishlists::app::item;
use crate::modules::whishlists::dtos::item::{CreateItem, UpdateItem};


// #[get("/")]
// pub fn get_items(state: &State<Repositories>) -> status::Custom<content::RawJson<String>> {
//   let manager  = item::Manager{
//     repo: Box::new(state.item.clone())
//   };

//   match manager.list() {
//     Ok(l) => {
//       let content = serde_json::to_string(&l).unwrap();
//       return status::Custom(Status::Ok, content::RawJson(content));
//     },
//     Err(_e) => { 
//       return status::Custom(Status::NotFound, content::RawJson(String::from("{}")));
//     }
//   }
// }

#[post("/<collection_id>", format="application/json", data="<create>")]
pub fn create_item(state: &State<AppState>, collection_id: String, create: Json<CreateItem>) -> status::Custom<content::RawJson<String>> {
  let manager  = item::Manager{
    repo: Box::new(state.repositories.item.clone())
  };

  let data = CreateItem {
    title: create.title.clone(),
    image: create.image.clone(),
    website: create.image.clone(),
    price: create.image.clone()
  };

  match manager.create(collection_id, data) {
    Ok(l) => {
      let content = serde_json::to_string(&l).unwrap();
      return status::Custom(Status::Created, content::RawJson(content));
    },
    Err(e) => {
      println!("{}", e);
      return status::Custom(Status::BadRequest, content::RawJson(String::from("{}")));
    }
  }
}

#[patch("/<id>", format="application/json", data="<partial>")]
pub fn update_item(state: &State<AppState>, id: String, partial: Json<UpdateItem>) -> status::Custom<content::RawJson<String>> {

  let manager  = item::Manager{
    repo: Box::new(state.repositories.item.clone())
  };

  let data = UpdateItem {
    title: partial.title.clone(),
    image: partial.image.clone(),
    website: partial.image.clone(),
    price: partial.image.clone(),
    obtained: partial.obtained.clone()
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
pub fn delete_item(state: &State<AppState>, id: String) -> status::Custom<content::RawJson<String>> {

  let manager  = item::Manager{
    repo: Box::new(state.repositories.item.clone())
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