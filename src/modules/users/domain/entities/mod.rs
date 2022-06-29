use serde::{Serialize, Deserialize};

/*
 * User
{
  "id": "e37f5ca9-fba1-46d9-a3ef-e80ada650784",
  "username": evesan",
  "active_subscription": true,
  "created_at": "2022-05-19T22:46:33.075Z"
}
*/

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
  pub id: String,
  pub username: String,
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
      active_subscription: true,
      created_at: String::from("2022-05-19T22:46:33.075Z")
    };

    assert_eq!(u.active_subscription, true);
  }
}