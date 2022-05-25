use api::infra::sync_mongo::Connection;
use api::modules::whishlists::{
  infra::repo::MongoCollectionRepo,
  app::ColectionManager
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
  let client = Connection::new().await;

  let repo = MongoCollectionRepo{
    client: client
  };

  let manager = ColectionManager {
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