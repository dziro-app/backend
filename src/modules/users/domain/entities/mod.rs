use serde::{Serialize, Deserialize};

/*
 * User
{
  "id": "e37f5ca9-fba1-46d9-a3ef-e80ada650784",
  "username": evesan",
  "profile_pic": "",
  "active_subscription": true,
  "created_at": "2022-05-19T22:46:33.075Z"
}
*/

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
  pub id: String,
  pub username: String,
  pub profile_pic: String,
  pub active_subscription: bool,
  pub created_at: String
}



#[cfg(test)]
mod users {
  use super::*;

  #[test]
  fn ut_test_item_entity() {
    let u = User {
      id: String::from("e37f5ca9-fba1-46d9-a3ef-e80ada650784"),
      username: String::from("Evesan"),
      profile_pic: String::from("https://www.amazon.com.mx/El-hombre-en-busca-sentido/dp/8425432022/ref=sr_1_1?adgrpid=1161084699107046&hvadid=72567867197970&hvbmt=be&hvdev=c&hvlocphy=119&hvnetw=s&hvqmt=e&hvtargid=kwd-72568145275092%3Aloc-119&hydadcr=27013_11422070&keywords=el+hombre+en+busca+de+sentido&qid=1652998524&sr=8-1"),
      active_subscription: true,
      created_at: String::from("2022-05-19T22:46:33.075Z")
    };

    assert_eq!(u.active_subscription, true);
  }
}