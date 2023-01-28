use mongodb::{bson::{doc, Bson}, options::{FindOneAndUpdateOptions, ReturnDocument}};

use crate::infra::sync_mongo::Connection;
use crate::modules::whishlists::domain::repositories::collection::CollectionRepository;
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

  fn list(&self, user_id: String) -> Result<Vec<Collection>, String> {
    let mut retrived_collections: Vec<Collection> = Vec::new();
    let collection = self.client.db.collection::<Collection>(COLLECTION_NAME);
    let cursor = match collection.find(doc!{"owner_id": user_id}, None) {
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

  fn list_shared(&self, user_id:String) -> Result<Vec<Collection>,String> {
    let mut retrived_collections: Vec<Collection> = Vec::new();

    let collection = self.client.db.collection::<Collection>(COLLECTION_NAME);
    let cursor = match collection.find(doc!{"shared_with.user_id": user_id, "shared_with.active": true}, None) {
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

  fn update(& self, user_id: String, id: String, data: UpdateCollection) -> Result<Collection, String> {
    let collection = self.client.db.collection::<Collection>(COLLECTION_NAME);
    let options = FindOneAndUpdateOptions::builder().return_document(ReturnDocument::After).build();


    let mut update = doc!{};
    let mut partial = doc! {};

    match data.name {
      Some(v) => { partial.insert("name", v);},
      None => {}
    };

    match data.color {
      Some(v) => { partial.insert("color", v);},
      None => {}
    };

    match data.emoji {
      Some(v) => { partial.insert("emoji", v);},
      None => {}
    };

    update.insert("$set", partial);

    match collection.find_one_and_update(doc! { "id": id, "owner_id": user_id }, update, options) {
      Ok(result) => {
        match result {
          Some(c) => { return Ok(c)},
          None => { return Err(String::from("Not found")) }
        }
      },
      Err(e) => {return Err(format!("{}", e))}
    }

  }

  fn share_with(&self, id: String, user_id: String, collaborator_id: String, can_edit: bool) -> Result<(),String> {

    let collection = self.client.db.collection::<Collection>(COLLECTION_NAME);
    
    let collaborators = doc!{
      "$push": {
        "shared_with": Bson::Document(doc!{
          "user_id": collaborator_id,
          "can_edit": can_edit,
          "active": true
        })
      }
    };

    match collection.find_one_and_update(doc!{"id": id, "owner_id": user_id}, collaborators, None) {
      Ok(result) => {
        match result {
          Some(_) => { return Ok(())},
          None => { return Err(String::from("Not found")) }
        }
      },
      Err(e) => { return Err(format!("{}", e))}
    }
  }

  fn delete(& self, user_id: String, id: String) -> Result<(), String> {
    let collection = self.client.db.collection::<Collection>(COLLECTION_NAME);
    match collection.delete_one(doc! { "id": id, "owner_id": user_id}, None) {
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
