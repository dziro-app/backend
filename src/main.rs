use rocket::{launch, routes, http::Method};
use rocket_cors::{AllowedOrigins};

use api::infra::state::{AppState, Repositories };
use api::infra::{config::Settings, sync_mongo::Connection};
use api::modules::{
  auth::infra::endpoints::{get_third_token,validate_third_token, refresh_token, clear_token},
  users::{infra::repo::MongoUserRepo}
};

use api::modules::whishlists::infra::{
  repo::{collection::MongoCollectionRepo, item::MongoItemRepo},
  endpoints::collection::{get_collections, create_collection, update_collection, delete_collection},
  endpoints::item::{scrap_item, create_item, update_item, toggle_obtained, delete_item}
};

use api::modules::api::infra::endpoints::get_version;

#[launch]
async fn rocket() ->  _ {
  // Read setting from env
  let settings = match Settings::new() {
    Ok(s) => {s},
    Err(e) => {panic!("{}", e)}
  };

  // Create Db connection
  let db = Connection::new(settings.database.host.clone(), settings.database.name.clone()).await;
  
  // Repositories
  let user_repo = MongoUserRepo { client: db.clone() };
  let collection_repo = MongoCollectionRepo{ client: db.clone() };
  let item_repo = MongoItemRepo{client: db};

  let repositories = Repositories {
    user: user_repo,
    collection: collection_repo,
    item: item_repo
  };



  let state = AppState {
    repositories: repositories,
    settings: settings.clone(),
  };

  let allowed_hosts:  Vec<&str> = settings.cors.allowed_hosts.split(", ").collect();
  let allowed_origins = AllowedOrigins::some_exact(&allowed_hosts);

  let cors = rocket_cors::CorsOptions {
    allowed_origins,

    allowed_methods: vec![Method::Get, Method::Options, Method::Post, Method::Patch, Method::Delete].into_iter().map(From::from).collect(),
    allow_credentials: true,
    ..Default::default()
  }
  .to_cors().expect("Bad cors config");


  let figment = rocket::Config::figment()
        .merge(("port", settings.server.port));
  
  rocket::custom(figment)
    .manage(state)
    .mount("/api/", routes![
      get_version
    ])
    .mount("/api/auth", routes![
      get_third_token,
      validate_third_token,
      refresh_token,
      clear_token
    ])
    .mount("/api/wishlists", routes![
      get_collections,
      create_collection,
      update_collection,
      delete_collection
    ])
    .mount("/api/items", routes![
      scrap_item,
      create_item,
      update_item,
      toggle_obtained,
      delete_item
    ])
    .attach(cors)
}
