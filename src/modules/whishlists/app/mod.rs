use uuid::Uuid;
use validator::{Validate};
use chrono::prelude::Local;

use super::{
  domain::entities::collection::Collection,
  dtos::collection::CreateCollection,
  domain::repository::CollectionRepository
};

pub struct ColectionManager {
  pub repo: Box<dyn CollectionRepository> 
}

impl ColectionManager {
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
}