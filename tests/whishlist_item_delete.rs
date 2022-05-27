use api::infra::sync_mongo::Connection;
use api::modules::whishlists::{
  infra::repo::item::MongoItemRepo,
  app::item,
};


#[tokio::test]
#[ignore = "mongodb disponibility"]
async fn it_delete_an_item() {
  /*
  * This test validates the connection with a mongo instace.
  - Uses a client connection
  - Uses a mongo repository
  - Uses CollectionManager struct
  */
  let client = Connection::new().await;

  let repo = MongoItemRepo{
    client: client
  };

  let manager = item::Manager {
    repo: Box::new(repo)
  };


  match manager.delete(String::from("654e0771-fa7f-4108-a326-ace4f72a39df")) {
    Ok(c) => {
      println!("{:#?}", c);
    },
    Err(e) => {
      panic!("{}", e);
    }
  };

}

