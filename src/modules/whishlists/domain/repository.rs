use crate::modules::whishlists::{
  domain::entities::collection::Collection,
  dtos::collection::{UpdateCollection}
};

pub trait CollectionRepository {
  fn list(&self) -> Result<Vec<Collection>, String>;
  fn save(&self, data: Collection) -> Result<Collection, String>;
  fn update(&self, id: String, data: UpdateCollection) -> Result<Collection, String>;
  fn delete(&self, id: String) -> Result<(), String>;
}