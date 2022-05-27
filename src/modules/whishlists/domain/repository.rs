use crate::modules::whishlists::{
  domain::entities::collection::Collection,
  dtos::collection::{UpdateCollection},
  domain::entities::item::Item,
  dtos::item::UpdateItem,
};


pub trait CollectionRepository {
  fn list(&self) -> Result<Vec<Collection>, String>;
  fn save(&self, data: Collection) -> Result<Collection, String>;
  fn update(&self, id: String, data: UpdateCollection) -> Result<Collection, String>;
  fn delete(&self, id: String) -> Result<(), String>;
}

pub trait ItemRepository {
  fn list(&self) -> Result<Vec<Item>, String>;
  fn save(&self, collection_id: String, data: Item) -> Result<Item, String>;
  fn update(&self, collection_id: String, id: String, data: UpdateItem) -> Result<Collection, String>;
  fn delete(&self, collection_id: String, id: String) -> Result<(), String>;
}