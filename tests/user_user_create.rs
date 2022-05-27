use api::infra::sync_mongo::Connection;
use api::modules::users::{
  dtos::CreateUser,
  infra::repo::MongoUserRepo,
  app::Manager,
};


#[tokio::test]
#[ignore = "mongodb disponibility"]
async fn it_create_a_user() {
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

  let data = CreateUser {
    id:  String::from("idfromspotify"),
    username: String::from("evesan"),
    profile_pic:  String::from("https://pbs.twimg.com/profile_images/1513637542561820672/W6ZJsVxV_400x400.jpg"),
  };

  match manager.create(data) {
    Ok(c) => {
      println!("{:#?}", c);
      assert_eq!(c.username, String::from("evesan"));
    },
    Err(e) => {
      panic!("{}", e);
    }
  };

}

