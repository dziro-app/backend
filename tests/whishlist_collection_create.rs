use api::infra::sync_mongo::Connection;
use api::infra::config::Settings;

use api::modules::whishlists::{
  dtos::collection::CreateCollection,
  infra::repo::collection::MongoCollectionRepo,
  app::collection,
};


#[tokio::test]
#[ignore = "mongodb disponibility"]
async fn it_create_a_collection() {
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

  let data = CreateCollection {
    name: String::from("Disfraces"),
    color: String::from("#00AF00"),
    emoji: String::from("⏰")
  };

  match manager.create(data) {
    Ok(c) => {
      println!("{:#?}", c);
      assert_eq!(c.name, String::from("Disfraces"));
    },
    Err(e) => {
      panic!("{}", e);
    }
  };

}

