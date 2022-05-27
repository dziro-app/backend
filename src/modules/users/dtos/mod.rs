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
  #[validate(url)]
  pub profile_pic: String
}


#[cfg(test)]
mod users {
  use super::*;

  #[test]
  fn ut_validate_user_item_dto() {
    let user = CreateUser {
      id: String::from("e37f5ca9-fba1-46d9-a3ef-e80ada650784"),
      username: String::from("Evesan"),
      profile_pic: String::from("https://www.amazon.com.mx/El-hombre-en-busca-sentido/dp/8425432022/ref=sr_1_1?adgrpid=1161084699107046&hvadid=72567867197970&hvbmt=be&hvdev=c&hvlocphy=119&hvnetw=s&hvqmt=e&hvtargid=kwd-72568145275092%3Aloc-119&hydadcr=27013_11422070&keywords=el+hombre+en+busca+de+sentido&qid=1652998524&sr=8-1"),
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