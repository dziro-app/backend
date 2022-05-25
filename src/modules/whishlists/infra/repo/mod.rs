use mongodb::{bson::doc};

use crate::infra::sync_mongo::Connection;
use crate::modules::whishlists::domain::repository::CollectionRepository;
use crate::modules::whishlists::{
  domain::entities::collection::Collection,
  dtos::collection::{UpdateCollection}
};

pub struct MongoCollectionRepo {
  pub client: Connection
}

static COLLECTION_NAME: &'static str = "collections";

// #[async_trait]
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

    fn update(& self, data: UpdateCollection) -> Result<Collection, String> {
        todo!()
    }

    fn delete(& self) -> Result<(), String> {
        todo!()
    }
}
