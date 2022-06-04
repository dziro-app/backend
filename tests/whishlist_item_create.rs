use api::infra::sync_mongo::Connection;
use api::infra::config::Settings;

use api::modules::whishlists::{
  dtos::item::CreateItem,
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
  let s = Settings::new().unwrap();
  let client =  Connection::new(s.database.host, s.database.name).await;

  let repo = MongoItemRepo{
    client: client
  };

  let manager = item::Manager {
    repo: Box::new(repo)
  };

  let data = CreateItem {
    title: String::from("Ropa"),
    image: String::from("https://m.media-amazon.com/images/P/B019H695T4.01._SCLZZZZZZZ_SX500_.jpg"),
    website: String::from("https://www.amazon.com.mx/El-hombre-en-busca-sentido/dp/8425432022/ref=sr_1_1?adgrpid=1161084699107046&hvadid=72567867197970&hvbmt=be&hvdev=c&hvlocphy=119&hvnetw=s&hvqmt=e&hvtargid=kwd-72568145275092%3Aloc-119&hydadcr=27013_11422070&keywords=el+hombre+en+busca+de+sentido&qid=1652998524&sr=8-1"),
    price: String::from("300.00")
  };

  match manager.create(String::from("00a3313b-c3a4-4bbf-ae94-8434a3bed088"), data) {
    Ok(c) => {
      println!("{:#?}", c);
      assert_eq!(c.title, String::from("Ropa"));
    },
    Err(e) => {
      panic!("{}", e);
    }
  };

}

