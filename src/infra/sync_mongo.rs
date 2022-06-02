use mongodb::{
  sync::Client,
  sync::Database
};

#[derive(Debug, Clone)]
pub struct Connection {
  pub db: Database
}

impl Connection {
  pub async fn new() -> Connection {
    let client = Client::with_uri_str("mongodb://root:example@localhost:27017").unwrap();

    let db = client.database("dziro");

    Connection{db: db}
  }
}

#[cfg(test)]
mod mongo {
  use super::*;

  #[test]
  fn ut_test_create_connection() {
    let _ = Connection::new();
  }

  #[tokio::test]
  #[ignore = "mongodb disponibility"]
  async fn ut_test_connection() {
    let c =  Connection::new().await ;

    let collections = match c.db.list_collection_names(None) {
      Ok(cs) => cs,
      Err(e) => {
        panic!("{}", e);
      }
    };

    for collection_name in collections {
      println!("😉 {}", collection_name);
    }
    println!("😉");
  }

}