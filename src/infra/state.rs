use crate::modules::{
  users::infra::repo::MongoUserRepo,
  whishlists::infra::repo::collection::MongoCollectionRepo,
  whishlists::infra::repo::item::MongoItemRepo
};

use crate::modules::auth::infra::spotify::SpotifyAuthConfig;


pub struct Repositories {
  pub user: MongoUserRepo,
  pub collection: MongoCollectionRepo,
  pub item: MongoItemRepo
}

#[derive(Clone)]
pub struct OauthsConfig {
  pub spotify: SpotifyAuthConfig
}

pub struct JwtConfig {
  pub secret: String
}


pub struct AppState {
  pub repositories: Repositories,
  pub oauths: OauthsConfig,
  pub jwt: JwtConfig
}