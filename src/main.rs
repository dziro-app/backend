use rocket::{launch, routes, build};
use api::infra::{config::Settings, sync_mongo::Connection};
use api::modules::{
  auth::infra::get_third_token,
  users::infra::repo::MongoUserRepo,
  whishlists::infra::repo::collection::MongoCollectionRepo,
  whishlists::infra::repo::item::MongoItemRepo
};


#[launch]
async fn rocket() -> _ {
  // Read setting from env
  let settings = match Settings::new() {
    Ok(s) => {s},
    Err(e) => {panic!("{}", e)}
  };

  // Create Db connection
  let db = Connection::new().await;
  
  // Repositories
  let uuser_repo = MongoUserRepo{client: db.clone()};
  let collection_repo = MongoCollectionRepo{client: db.clone()};
  let item_repo = MongoItemRepo{client: db};

  
  build()
    .mount("/api/auth", routes![
      get_third_token
    ])
}
