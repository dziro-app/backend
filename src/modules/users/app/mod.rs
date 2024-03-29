use validator::{Validate};
use chrono::prelude::Local;

use crate::modules::users::{
  dtos::CreateUser,
  domain::entities::User,
  domain::{repository::UserRepositoy}
};

pub struct Manager {
  pub repo: Box<dyn UserRepositoy>
}

impl Manager {
  pub fn create(&self, data: CreateUser) -> Result<User, String> {
    match data.validate() {
      Ok(_) => {},
      Err(e) => {
        panic!("Validation err {}", e)
      }
    };

    let user = User {
      id: data.id,
      username: data.username,
      email: data.email,
      active_subscription: false,
      created_at: String::from(Local::now().to_string())
    };

    match self.repo.save(user) {
      Ok(u) => { Ok(u) },
      Err(e) => { return Err(format!("{}", e))}
    }
  }

  pub fn find(&self, email: String) -> Result<User, String> {
    match self.repo.find_by_email(email) {
      Some(u) => {Ok(u)},
      None => { return Err("Not found".to_string())}
    }
  }
}