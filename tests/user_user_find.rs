use api::infra::sync_mongo::Connection;
use api::modules::users::{
  infra::repo::MongoUserRepo,
  app::Manager,
};


#[tokio::test]
#[ignore = "mongodb disponibility"]
async fn it_finds_a_user() {
  /*
  * This test validates the connection with a mongo instace.
  - Uses a client connection
  - Uses a mongo repository
  - Uses UserManager struct
  */
  let client = Connection::new().await;

  let repo = MongoUserRepo{
    client: client
  };

  let manager = Manager {
    repo: Box::new(repo)
  };

  match manager.find(String::from("79e24c13-e510-4f26-aa51-3a985e6c6bfd")) {
    Ok(c) => {
      println!("{:#?}", c);
      assert_eq!(c.username, String::from("evesan"));
    },
    Err(e) => {
      panic!("{}", e);
    }
  };

}

