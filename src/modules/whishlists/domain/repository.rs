use crate::modules::whishlists::{
  domain::entities::collection::Collection,
  dtos::collection::{UpdateCollection},
  domain::entities::item::Item,
  dtos::item::UpdateItem,
};


pub trait CollectionRepository {
  fn list(&self, user_id: String) -> Result<Vec<Collection>, String>;
  fn save(&self, data: Collection) -> Result<Collection, String>;
  fn update(&self, user_id: String, id: String, data: UpdateCollection) -> Result<Collection, String>;
  fn delete(&self, user_id: String, id: String) -> Result<(), String>;
}

pub trait ItemRepository {
  fn find(&self, user_id: String, id: String) -> Result<Item, String>;
  fn save(&self, user_id: String, collection_id: String, data: Item) -> Result<Item, String>;
  fn update(&self, user_id: String, id: String, data: UpdateItem) -> Result<Item, String>;
  fn delete(&self, user_id: String, id: String) -> Result<(), String>;
}