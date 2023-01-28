use crate::modules::users::domain::entities::User;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg(test)]
use crate::modules::users::{
  dtos::CreateUser,
  app::Manager,
};

#[cfg_attr(test, automock)]
pub trait UserRepositoy {
  fn save(&self, user: User) -> Result<User, String>;
  fn find(&self, id: String) -> Option<User>;
  fn find_by_email(&self, email: String) -> Option<User>;
}

#[cfg(test)]
mod user_repository {
  use super::*;

  #[test]
  fn ut_create_a_user() {
    /*
    * This test validates the repositories are used with correctly
    - Uses a mock repository
    - Uses UserManager struct
    */
  
    let mut repo = MockUserRepositoy::new();

    repo.expect_save()
      .times(1)
      .returning(|x| Ok(x) );
  
    let manager = Manager {
      repo: Box::new(repo)
    };
  
    let data = CreateUser {
      id:  String::from("idfromspotify"),
      email: String::from("email from spotify"),
      username: String::from("evesan"),
    };
  
    match manager.create(data) {
      Ok(c) => {
        assert_eq!(c.username, String::from("evesan"));
      },
      Err(e) => {
        panic!("{}", e);
      }
    };
  
  }

  #[test]
  fn ut_finds_a_user() {
    /*
    * This test validates the connection with a mongo instace.
    - Uses a mock repository
    - Uses UserManager struct
    */

    let mut repo = MockUserRepositoy::new();

    repo.expect_find()
      .times(1)
      .returning(|_x| Some(User{
        id: String::from(""),
        email: String::from(""),
        username: String::from("evesan"),
        active_subscription: false,
        created_at: String::from("")
      })
    );
  
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

}

