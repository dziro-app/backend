use api::infra::sync_mongo::Connection;
use api::modules::whishlists::{
  dtos::item::UpdateItem,
  infra::repo::item::MongoItemRepo,
  app::item,
};


#[tokio::test]
#[ignore = "mongodb disponibility"]
async fn it_create_an_item() {
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

  let data = UpdateItem {
    image: None,
    title: None,
    website: None,
    price: None,
    obtained: Some(true)
  };

  match manager.update(
    String::from("91442251-34ff-4cbd-9de9-8657418897f3"),
    String::from("97b752ea-3f37-4107-923e-c56645c3d7b6"), 
    data) {
    Ok(c) => {
      println!("{:#?}", c);
      // assert_eq!(c.title, String::from("Ropa"));
    },
    Err(e) => {
      panic!("{}", e);
    }
  };

}

