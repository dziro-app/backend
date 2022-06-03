use mongodb::{bson::doc, options::{FindOneAndUpdateOptions, ReturnDocument}};

use crate::infra::sync_mongo::Connection;
use crate::modules::whishlists::domain::repository::CollectionRepository;
use crate::modules::whishlists::{
  domain::entities::collection::Collection,
  dtos::collection::{UpdateCollection}
};


pub static COLLECTION_NAME: &'static str = "collections";

#[derive(Clone)]
pub struct MongoCollectionRepo {
  pub client: Connection
}


impl CollectionRepository for MongoCollectionRepo {

  fn list(&self) -> Result<Vec<Collection>, String> {
    let mut retrived_collections: Vec<Collection> = Vec::new();
    let collection = self.client.db.collection::<Collection>(COLLECTION_NAME);
    let cursor = match collection.find(doc! { }, None) {
      Ok(c) => {c},
      Err(e) => {
        return Err(format!("{}", e));
      }
    };

    for result in cursor {
      let collection = match result {
        Ok(r) => {r},
        Err(e) => { return Err(format!("{}", e));}
      };
      retrived_collections.push(collection);        
    }

    return Ok(retrived_collections);
  }

  fn save(& self, data: Collection) -> Result<Collection, String> {

    let collection = self.client.db.collection::<Collection>(COLLECTION_NAME);

    match collection.insert_one(data.clone(), None) {
      Ok(_) => {
        return Ok(data);
      },
      Err(e) => { return Err(format!("{}", e));}
    };

  }

  fn update(& self, id: String, data: UpdateCollection) -> Result<Collection, String> {
    let collection = self.client.db.collection::<Collection>(COLLECTION_NAME);
    let options = FindOneAndUpdateOptions::builder().return_document(ReturnDocument::After).build();

    let update= doc! {
      "$set": {
        "name": data.name,
        "color": data.color,
        "emoji": data.emoji,
     }
    };

    match collection.find_one_and_update(doc! { "id": id }, update, options) {
      Ok(result) => {
        match result {
          Some(c) => { return Ok(c)},
          None => { return Err(String::from("Not found")) }
        }
      },
      Err(e) => {return Err(format!("{}", e))}
    }

  }

  fn delete(& self, id: String) -> Result<(), String> {
    let collection = self.client.db.collection::<Collection>(COLLECTION_NAME);
    match collection.delete_one(doc! { "id": id}, None) {
      Ok(r) => {
        if r.deleted_count > 0 {
          return Ok(())
        }
        return Err(String::from("Not found"))
      },
      Err(e) => {return Err(format!("{}", e))}
    };
  }
}
