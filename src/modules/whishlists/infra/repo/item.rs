use mongodb::{bson::{doc, Bson}, options::{FindOneAndUpdateOptions, ReturnDocument}};

use crate::infra::sync_mongo::Connection;
use crate::modules::whishlists::domain::repository::ItemRepository;
use crate::modules::whishlists::{
  domain::entities::collection::Collection,
  domain::entities::item::Item,
  dtos::item::UpdateItem
};

use super::collection::COLLECTION_NAME;

#[derive(Clone)]
pub struct MongoItemRepo {
  pub client: Connection
}

impl ItemRepository for MongoItemRepo {

  fn find(&self, user_id: String, id: String) -> Result<Item, String> {
    let collection = self.client.db.collection::<Collection>(COLLECTION_NAME);
    match collection.find_one(doc! {"owner_id": user_id,  "items.id": id.clone() }, None).unwrap() {
      Some(c) => {
        let updated: Vec<Item>= c.items
          .into_iter()
          .filter(|item| item.id == id)
          .collect();
        
        return Ok(updated[0].clone());
      },
      None => {return Err(format!("Not found"))}
    }
  }

  fn save(&self, user_id: String, collection_id: String, data: Item) -> Result<Item, String> {
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

    match collection.find_one_and_update(doc! { "id": collection_id, "owner_id": user_id}, update, None) {
      Ok(result) => {
        match result {
          Some(_) => { return Ok(return_data)},
          None => { return Err(String::from("Not found")) }
        }
      },
      Err(e) => {return Err(format!("{}", e))}
    }
  }

  fn update(&self, user_id: String,  id: String, data: UpdateItem) -> Result<Item, String> {
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
      Some(v) => { partial.insert("items.$.price", v.parse::<f32>().unwrap()); },
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
    
    match collection.find_one_and_update(doc! { "owner_id": user_id, "items.id": id.clone() }, update, options).unwrap() {
      Some(c) => {
        let updated: Vec<Item>= c.items
          .into_iter()
          .filter(|item| item.id == id)
          .collect();
        
        return Ok(updated[0].clone());
      },
      None => {return Err(format!("Not found"))}
    }
  }


  fn delete(&self, user_id: String, id: String) -> Result<(), String> {
    let collection = self.client.db.collection::<Collection>(COLLECTION_NAME);
    let options = FindOneAndUpdateOptions::builder()
      .return_document(ReturnDocument::After)
      .build();

    let delete = doc! {
      "$pull": { "items": { "id": id.clone() }}
    };


    match collection.find_one_and_update(doc! { "owner_id": user_id, "items.id": id }, delete, options).unwrap() {
      Some(_) => {
        return Ok(());
      },
      None => {return Err(format!("Not found"))}
    }
  }
}