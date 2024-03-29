use uuid::Uuid;
use validator::{Validate};
use chrono::prelude::Local;

use crate::modules::whishlists::{
  domain::entities::item::Item,
  dtos::item::{CreateItem, UpdateItem},
  domain::{repositories::item::ItemRepository}
};

pub struct Manager {
  pub repo: Box<dyn ItemRepository>,
  pub user_id: String
}

impl Manager {
  pub fn create(&self, collection_id: String, data: CreateItem) -> Result<Item, String> {
    // Validates required data
    match data.validate() {
      Ok(_) => {},
      Err(e) => {
        return Err(format!("{}", e));
      }
    };

    let new_collection = Item {
      id: String::from(Uuid::new_v4().to_string()),
      title: data.title,
      website: data.website,
      price: data.price.parse::<f32>().unwrap(),
      image: data.image,
      obtained: false,
      created_at: String::from(Local::now().to_string())
    };

    match self.repo.save(self.user_id.clone(), collection_id, new_collection) {
      Ok(d) => {return Ok(d);},
      Err(e) => {
        return Err(format!("{}", e));
      }
    }
  }

  pub fn update(&self, id: String, data: UpdateItem) -> Result<Item, String> {
    match data.validate() {
      Ok(_) => {},
      Err(e) => {
        return Err(format!("{}", e));
      }
    };
    
    match self.repo.update(self.user_id.clone(), id, data) {
      Ok(updated) => {return Ok(updated)},
      Err(e) => {
        // todo: replace with logger lib
        println!("{}", e);
        return Err(e);
      }
    }
  }

  pub fn toggle_obtained(&self, id: String) -> Result<Item, String> {
    let original = match self.repo.find(self.user_id.clone(), id) {
      Ok(found) => {found},
      Err(e) => {
        // todo: replace with logger lib
        println!("{}", e);
        return Err(e);
      }
    };

    let update = UpdateItem {
      title: None,
      image: None,
      website: None,
      price: None,
      obtained: Some(!original.obtained)
    };

    match self.repo.update(self.user_id.clone(), original.id, update) {
      Ok(updated) => {return Ok(updated)},
      Err(e) => {
        // todo: replace with logger lib
        println!("{}", e);
        return Err(e);
      }
    }

  }

  pub fn delete(&self, id: String) -> Result<(), String> {
    match self.repo.delete(self.user_id.clone(), id) {
      Ok(_) => { Ok (())},
      Err(e) => {
        // todo: replace with logger lib
        println!("{}", e);
        return Err(e);
      }
    }
  }
}