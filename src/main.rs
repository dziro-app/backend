use rocket::{launch, routes, build};

use api::infra::state::Repositories;
use api::infra::{config::Settings, sync_mongo::Connection};
use api::modules::{
  auth::infra::get_third_token,
  users::{infra::repo::MongoUserRepo}
};

use api::modules::whishlists::infra::{
  repo::{collection::MongoCollectionRepo, item::MongoItemRepo},
  endpoints::collection::{get_collections, create_collection, update_collection, delete_collection}
};


#[launch]
async fn rocket() -> _ {
  // Read setting from env
  let settings = match Settings::new() {
    Ok(s) => {s},
    Err(e) => {panic!("{}", e)}
  };

  // Create Db connection
  let db = Connection::new(settings.database.host, settings.database.name).await;
  
  // Repositories
  let user_repo = MongoUserRepo { client: db.clone() };
  let collection_repo = MongoCollectionRepo{ client: db.clone() };
  let item_repo = MongoItemRepo{client: db};

  let repositories = Repositories {
    user: user_repo,
    collection: collection_repo,
    item: item_repo
  };
  
  build()
    .manage(repositories)
    .mount("/api/auth", routes![
      get_third_token
    ])
    .mount("/api/wishlists", routes![
      get_collections,
      create_collection,
      update_collection,
      delete_collection
    ])
}
