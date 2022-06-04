use api::infra::sync_mongo::Connection;
use api::infra::config::Settings;

use api::modules::whishlists::{
  infra::repo::collection::MongoCollectionRepo,
  app::collection,
};

#[tokio::test]
#[ignore = "mongodb disponibility"]
async fn it_read_collections() {
  /*
  * This test validates the connection with a mongo instace.
  - Uses a client connection
  - Uses a mongo repository
  - Uses CollectionManager struct
  */
  let s = Settings::new().unwrap();
  let client =  Connection::new(s.database.host, s.database.name).await;

  let repo = MongoCollectionRepo{
    client: client
  };

  let manager = collection::Manager {
    repo: Box::new(repo)
  };

  match manager.list() {
    Ok(cs) => {
      println!("{:#?}", cs);
    }, 
    Err(e) => {
      panic!("{}", e);
    }
  };
}