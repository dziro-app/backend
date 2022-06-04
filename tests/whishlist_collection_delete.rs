use api::infra::sync_mongo::Connection;
use api::infra::config::Settings;

use api::modules::whishlists::{
  infra::repo::collection::MongoCollectionRepo,
  app::collection,
};


#[tokio::test]
#[ignore = "mongodb disponibility"]
async fn it_delete_a_collection() {
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

  match manager.delete(String::from("")) {
    Ok(_) => {},
    Err(e) => {
      panic!("{}", e);
    }
  };

}

