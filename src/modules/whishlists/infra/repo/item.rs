use mongodb::{bson::{doc, Bson}, options::{FindOneAndUpdateOptions, ReturnDocument}};

use crate::infra::sync_mongo::Connection;
use crate::modules::whishlists::domain::repository::ItemRepository;
use crate::modules::whishlists::{
  domain::entities::collection::Collection,
  domain::entities::item::Item,
  dtos::item::UpdateItem
};

use super::collection::COLLECTION_NAME;

pub struct MongoItemRepo {
  pub client: Connection
}

impl ItemRepository for MongoItemRepo {

  fn list(&self) -> Result<Vec<Item>, String> {
    todo!();
  }

  fn save(&self, collection_id: String, data: Item) -> Result<Item, String> {
    let collection = self.client.db.collection::<Collection>(COLLECTION_NAME);

    let return_data = data.clone();

    let update = doc!{
      "$push": {
        "items": Bson::Document(doc!{
          "id": data.id,
          "title": data.title,
          "website": data.website,
          "price": data.price,
          "image": data.image,
          "obtained": data.obtained,
          "created_at": data.created_at
        })
      }
    };

    match collection.find_one_and_update(doc! { "id": collection_id }, update, None) {
      Ok(result) => {
        match result {
          Some(_) => { return Ok(return_data)},
          None => { return Err(String::from("Not found")) }
        }
      },
      Err(e) => {return Err(format!("{}", e))}
    }
  }

  fn update(&self, id: String, data: UpdateItem) -> Result<Collection, String> {
    let collection = self.client.db.collection::<Collection>(COLLECTION_NAME);
    let options = FindOneAndUpdateOptions::builder()
      .return_document(ReturnDocument::After)
      .build();


    let mut update = doc!{};
    let mut partial = doc! {};

    match data.title {
      Some(v) => { partial.insert("items.$.title", v);},
      None => {}
    };

    match data.website {
      Some(v) => { partial.insert("items.$.website", v);},
      None => {}
    };

    match data.price {
      Some(v) => { partial.insert("items.$.price", v);},
      None => {}
    };

    match data.image {
      Some(v) => { partial.insert("items.$.image", v);},
      None => {}
    };

    match data.obtained {
      Some(v) => { partial.insert("items.$.obtained", v);},
      None => {}
    };

    update.insert("$set", partial);

    
    match collection.find_one_and_update(doc! { "items.id": id }, update, options).unwrap() {
      Some(c) => {
        // let updated: Vec<Item>= c.items.iter().filter(|item| item.id == id).collect();
        return Ok(c);
      },
      None => {return Err(format!("Not found"))}
    }
  }

  fn delete(&self, collection_id: String, id: String) -> Result<(), String> {
    todo!();
  }
}