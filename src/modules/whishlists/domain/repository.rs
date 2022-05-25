use crate::modules::whishlists::{
  domain::entities::collection::Collection,
  dtos::collection::{UpdateCollection}
};

pub trait CollectionRepository {
  fn list(&self) -> Result<Vec<Collection>, String>;
  fn save(&self, data: Collection) -> Result<Collection, String>;
  fn update(&self, data: UpdateCollection) -> Result<Collection, String>;
  fn delete(&self) -> Result<(), String>;
}