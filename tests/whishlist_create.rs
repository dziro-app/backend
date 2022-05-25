use api::infra::sync_mongo::Connection;
use api::modules::whishlists::{
  infra::repo::MongoCollectionRepo,
  app::ColectionManager,
  dtos::collection::CreateCollection
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
  let client = Connection::new().await;

  let repo = MongoCollectionRepo{
    client: client
  };

  let manager = ColectionManager {
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

