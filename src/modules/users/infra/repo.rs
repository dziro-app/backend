use mongodb::{bson::doc};

use crate::infra::sync_mongo::Connection;

use crate::modules::users::domain::repository::UserRepositoy;
use crate::modules::users::{
  domain::entities::User
};


pub static COLLECTION_NAME: &'static str = "users";


pub struct MongoUserRepo {
  pub client: Connection
}

impl UserRepositoy for MongoUserRepo {
  fn save(&self, user: User) -> Result<User, String> {
    let collection = self.client.db.collection::<User>(COLLECTION_NAME);

    match collection.insert_one(user.clone(), None) {
      Ok(_) => {
        return Ok(user);
      },
      Err(e) => { return Err(format!("{}", e));}
    };  
  }

  fn find(&self, id: String) -> Result<User, String> {
    let collection = self.client.db.collection::<User>(COLLECTION_NAME);
    match collection.find_one(doc!{ "id": id }, None).unwrap() {
      Some(user) => {
        return Ok(user);
      },
      None => { return Err(String::from("Not found"));}
    };
  }
}