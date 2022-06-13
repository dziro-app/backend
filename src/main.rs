use rocket::{launch, routes, build, http::Method};
use rocket_cors::{AllowedOrigins};

use api::infra::state::{AppState, Repositories, OauthsConfig, JwtConfig};
use api::infra::{config::Settings, sync_mongo::Connection};
use api::modules::{
  auth::infra::endpoints::{get_third_token,validate_third_token},
  auth::infra::spotify::SpotifyAuthConfig,
  users::{infra::repo::MongoUserRepo}
};

use api::modules::whishlists::infra::{
  repo::{collection::MongoCollectionRepo, item::MongoItemRepo},
  endpoints::collection::{get_collections, create_collection, update_collection, delete_collection},
  endpoints::item::{create_item, update_item, toggle_obtained, delete_item}
};


#[launch]
async fn rocket() ->  _ {
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

  let spotify_settings = SpotifyAuthConfig {
    client: settings.spotify.client,
    secret: settings.spotify.secret,
    callback: settings.spotify.callback
  };

  let state = AppState {
    repositories: repositories,
    oauths: OauthsConfig{
      spotify: spotify_settings
    },
    jwt: JwtConfig {
      secret: settings.jwt.secret
    }
  };

  let allowed_origins = AllowedOrigins::some_exact(&[
    "http://localhost:3000",
    "http://192.168.100.70:3000"
  ]);

  let cors = rocket_cors::CorsOptions {
    allowed_origins,

    allowed_methods: vec![Method::Get, Method::Options, Method::Post, Method::Patch, Method::Delete].into_iter().map(From::from).collect(),
    // allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
    allow_credentials: true,
    ..Default::default()
  }
  .to_cors().expect("Bad cors config");

  
  build()
    .manage(state)
    .mount("/api/auth", routes![
      get_third_token,
      validate_third_token
    ])
    .mount("/api/wishlists", routes![
      get_collections,
      create_collection,
      update_collection,
      delete_collection
    ])
    .mount("/api/items", routes![
      create_item,
      update_item,
      toggle_obtained,
      delete_item
    ])
    .attach(cors)
}
