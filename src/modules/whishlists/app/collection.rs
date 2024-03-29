use uuid::Uuid;
use validator::{Validate};
use chrono::prelude::Local;

use crate::modules::whishlists::{
  domain::entities::collection::Collection,
  dtos::collection::{CreateCollection, UpdateCollection},
  domain::repositories::collection::CollectionRepository
};

pub struct Manager {
  pub repo: Box<dyn CollectionRepository>,
  pub user_id: String
}

impl Manager {
  pub fn list(&self) -> Result<Vec<Collection>, String> {

    let mut collections = match self.repo.list(self.user_id.clone()) {
      Ok(c) => {c},
      Err(e) => {
        // todo: replace with logger lib
        println!("{}", e);
        return Err(e);
      }
    };
    let shared = match self.repo.list_shared(self.user_id.clone()) {
      Ok(c) => {c},
      Err(e) => {
        // todo: replace with logger lib
        println!("{}", e);
        return Err(e);
      }
    };

    collections.extend(shared);

    return Ok(collections);
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
      shared_with: Vec::new(),
      owner_id: self.user_id.clone(),
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
    
    match self.repo.update(self.user_id.clone(), id, data) {
      Ok(updated) => {return Ok(updated)},
      Err(e) => {
        // todo: replace with logger lib
        println!("{}", e);
        return Err(e);
      }
    }
  }

  pub fn add_collaborator(&self, id: String, collaborator_id: String, can_edit: bool) -> Result<(), String> {
    
    match self.repo.share_with( id, self.user_id.clone(), collaborator_id, can_edit) {
      Ok(_) => { Ok (())},
      Err(e) => {
        return Err(e)
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