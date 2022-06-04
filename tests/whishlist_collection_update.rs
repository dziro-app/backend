use api::infra::sync_mongo::Connection;
use api::infra::config::Settings;

use api::modules::whishlists::{
  infra::repo::collection::MongoCollectionRepo,
  app::collection,
  dtos::collection::UpdateCollection
};


#[tokio::test]
#[ignore = "mongodb disponibility"]
async fn it_update_a_collection() {
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


  let new_data = UpdateCollection {
    name: Some(String::from("Ropa6")),
    color: Some(String::from("#00ff89")),
    emoji: Some(String::from("R"))
  };


  let n = manager.update(String::from(""), new_data).unwrap();

  assert_eq!(n.name, String::from("Ropa6"));

}


