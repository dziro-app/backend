use crate::modules::{
  users::infra::repo::MongoUserRepo,
  whishlists::infra::repo::collection::MongoCollectionRepo,
  whishlists::infra::repo::item::MongoItemRepo
};


pub struct Repositories {
  pub user: MongoUserRepo,
  pub collection: MongoCollectionRepo,
  pub item: MongoItemRepo
}