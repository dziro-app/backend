use mongodb::{Client, Database, options::ClientOptions, error::Error};

#[derive(Debug)]
pub struct Connection {
  pub db: Database
}

impl Connection {
  pub async fn new() -> Result<Connection, Error> {
    let client_options = ClientOptions::parse("mongodb://root:example@localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("dziro");

    return Ok(Connection{db: db});

  }
}

#[cfg(test)]
mod mongo {
  use super::*;

  #[tokio::test]
  async fn ut_test_create_connection() {
    match Connection::new().await {
      Ok(_) => {
        assert!(true);
      }, 
      Err(e) => {
        panic!("{}", e);
      }
    }
  }

  #[tokio::test]
  #[ignore]
  async fn ut_test_connection() {
    let c = match Connection::new().await {
      Ok(client) => client, 
      Err(e) => {
        panic!("{}", e);
      }  
    };

    let collections = match c.db.list_collection_names(None).await {
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