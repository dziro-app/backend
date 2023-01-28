use serde::{Deserialize, Serialize};
use validator::{Validate};

/*
 * User data transfer object
*/

#[derive(Deserialize, Serialize, Validate, Clone)]
pub struct CreateUser {
  #[validate(length(min = 1))]
  pub id: String,
  #[validate(length(min = 1))]
  pub username: String,
  #[validate(length(min = 1))]
  pub email: String
}


#[cfg(test)]
mod users {
  use super::*;

  #[test]
  fn ut_validate_user_item_dto() {
    let user = CreateUser {
      id: String::from("e37f5ca9-fba1-46d9-a3ef-e80ada650784"),
      username: String::from("Evesan"),
      email: String::from("everardo.ipn@gmail.com")
    };

    match user.validate() {
      Ok(_) => {
        assert_eq!(user.username, String::from("Evesan"))
      },
      Err(e) => {
        panic!("Validation err {}", e)
      }
    }
  }
}