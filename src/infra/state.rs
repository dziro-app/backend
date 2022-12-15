use crate::modules::{
  users::infra::repo::MongoUserRepo,
  whishlists::infra::repo::collection::MongoCollectionRepo,
  whishlists::infra::repo::item::MongoItemRepo
};

use crate::infra::config::Settings;

pub struct Repositories {
  pub user: MongoUserRepo,
  pub collection: MongoCollectionRepo,
  pub item: MongoItemRepo
}


pub struct AppState {
  pub repositories: Repositories,
  pub settings: Settings,
}