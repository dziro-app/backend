use create::modules::whishlists::{
  domain::entities::collection::Collection,
  dtos::CreateCollection
};

pub trait Collection {
  fn save(& self, data: CreateCollection) -> Result<Collection, Error>;
}