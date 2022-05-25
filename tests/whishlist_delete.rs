use api::infra::sync_mongo::Connection;
use api::modules::whishlists::{
  infra::repo::MongoCollectionRepo,
  app::ColectionManager,
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
  let client = Connection::new().await;

  let repo = MongoCollectionRepo{
    client: client
  };

  let manager = ColectionManager {
    repo: Box::new(repo)
  };

  match manager.delete(String::from("")) {
    Ok(_) => {},
    Err(e) => {
      panic!("{}", e);
    }
  };

}

