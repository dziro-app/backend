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

  fn update(&self, collection_id: String, id: String, data: UpdateItem) -> Result<Collection, String> {
    let collection = self.client.db.collection::<Collection>(COLLECTION_NAME);
    let arr_f = Some(vec![doc!{ "id": { "$eq":  id} }]);
    let options = FindOneAndUpdateOptions::builder()
      .return_document(ReturnDocument::After)
      .array_filters(arr_f) .build();

    let update = doc!{
      "$set": {
        "items.$[id]": Bson::Document(doc!{
          "title": data.title,
          "website": data.website,
          "price": data.price,
          "image": data.image,
          "obtained": data.obtained,
        })
      }
    };
    
    match collection.find_one_and_update(doc! { "id": collection_id }, update, options).unwrap() {
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