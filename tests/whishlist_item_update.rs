use api::infra::sync_mongo::Connection;
use api::infra::config::Settings;

use api::modules::whishlists::{
  dtos::item::UpdateItem,
  infra::repo::item::MongoItemRepo,
  app::item,
};


#[tokio::test]
#[ignore = "mongodb disponibility"]
async fn it_updates_an_item() {
  /*
  * This test validates the connection with a mongo instace.
  - Uses a client connection
  - Uses a mongo repository
  - Uses CollectionManager struct
  */
  let s = Settings::new().unwrap();
  let client =  Connection::new(s.database.host, s.database.name).await;

  let repo = MongoItemRepo{
    client: client
  };

  let manager = item::Manager {
    repo: Box::new(repo)
  };

  let data = UpdateItem {
    image: None,
    title: Some(String::from("El hombre en busca de sentido")),
    website: None,
    price: None,
    obtained: Some(true)
  };

  match manager.update(
    String::from("1defe591-2dd7-4252-804c-7eae006ce1b9"), 
    data) {
    Ok(c) => {
      println!("{:#?}", c);
    },
    Err(e) => {
      panic!("{}", e);
    }
  };

}

