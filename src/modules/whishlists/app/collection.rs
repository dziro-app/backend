use uuid::Uuid;
use validator::{Validate};
use chrono::prelude::Local;

use crate::modules::whishlists::{
  domain::entities::collection::Collection,
  dtos::collection::{CreateCollection, UpdateCollection},
  domain::repository::CollectionRepository
};

pub struct Manager {
  pub repo: Box<dyn CollectionRepository> 
}

impl Manager {
  pub fn list(&self) -> Result<Vec<Collection>, String> {
    match self.repo.list() {
      Ok(c) => {return Ok(c)},
      Err(e) => {
        // todo: replace with logger lib
        println!("{}", e);
        return Err(e);
      }
    }
  }

  pub fn create(&self, data: CreateCollection) -> Result<Collection, String> {
    // Validates required data
    match data.validate() {
      Ok(_) => {},
      Err(e) => {
        return Err(format!("{}", e));
      }
    };

    let new_collection = Collection {
      id: String::from(Uuid::new_v4().to_string()),
      name: data.name,
      color: data.color,
      emoji: data.emoji,
      items: Vec::new(),
      owner_id: String::from(""),
      created_at: String::from(Local::now().to_string())
    };

    match self.repo.save(new_collection) {
      Ok(d) => {return Ok(d);},
      Err(e) => {
        return Err(format!("{}", e));
      }
    }
  }

  pub fn update(&self, id: String, data: UpdateCollection) -> Result<Collection, String> {
    match data.validate() {
      Ok(_) => {},
      Err(e) => {
        return Err(format!("{}", e));
      }
    };
    
    match self.repo.update(id, data) {
      Ok(updated) => {return Ok(updated)},
      Err(e) => {
        // todo: replace with logger lib
        println!("{}", e);
        return Err(e);
      }
    }
  }

  pub fn delete(&self, id: String) -> Result<(), String> {
    match self.repo.delete(id) {
      Ok(_) => { Ok (())},
      Err(e) => {
        // todo: replace with logger lib
        println!("{}", e);
        return Err(e);
      }
    }
  }
}