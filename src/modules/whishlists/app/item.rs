use uuid::Uuid;
use validator::{Validate};
use chrono::prelude::Local;

use crate::modules::whishlists::{
  domain::entities::item::Item,
  dtos::item::{CreateItem, UpdateItem},
  domain::{repository::ItemRepository, entities::collection::Collection}
};

pub struct Manager {
  pub repo: Box<dyn ItemRepository> 
}

impl Manager {
  pub fn list(&self) -> Result<Vec<Item>, String> {
    match self.repo.list() {
      Ok(c) => {return Ok(c)},
      Err(e) => {
        // todo: replace with logger lib
        println!("{}", e);
        return Err(e);
      }
    }
  }

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

    match self.repo.save(collection_id, new_collection) {
      Ok(d) => {return Ok(d);},
      Err(e) => {
        return Err(format!("{}", e));
      }
    }
  }

  pub fn update(&self, collection_id: String, id: String, data: UpdateItem) -> Result<Collection, String> {
    match data.validate() {
      Ok(_) => {},
      Err(e) => {
        return Err(format!("{}", e));
      }
    };
    
    match self.repo.update(collection_id, id, data) {
      Ok(updated) => {return Ok(updated)},
      Err(e) => {
        // todo: replace with logger lib
        println!("{}", e);
        return Err(e);
      }
    }
  }

  pub fn delete(&self, collection_id: String, id: String) -> Result<(), String> {
    match self.repo.delete(collection_id, id) {
      Ok(_) => { Ok (())},
      Err(e) => {
        // todo: replace with logger lib
        println!("{}", e);
        return Err(e);
      }
    }
  }
}