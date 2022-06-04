use mongodb::{
  sync::Client,
  sync::Database
};

#[derive(Debug, Clone)]
pub struct Connection {
  pub db: Database
}

impl Connection {
  pub async fn new(uri: String, db_name: String) -> Connection {
    let client = Client::with_uri_str(uri).unwrap();

    let db = client.database(db_name.as_str());

    Connection{db: db}
  }
}

#[cfg(test)]
mod mongo {
  use super::*;
  use crate::infra::config::Settings;

  #[test]
  fn ut_test_create_connection() {
    let s = Settings::new().unwrap();
    let _ = Connection::new(s.database.host, s.database.name);
  }

  #[tokio::test]
  #[ignore = "mongodb disponibility"]
  async fn ut_test_connection() {
    let s = Settings::new().unwrap();

    let c =  Connection::new(s.database.host, s.database.name).await ;

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