use serde::{Serialize, Deserialize};
use crate::modules::whishlists::domain::entities::item::Item;

/*
* Collection
{
 "id": "14c97177-33c4-4e41-8af6-94241526a7c4",
 "name": "Libros",
 "color": "#eaa510"
 "emoji": "📖",
 "ownerId": 0,
 "collaborators": [],
 "items": [],
 "createdAt": "2022-04-18T22:39:16.627Z",
}
*/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Collection {
  pub id: String,
  pub name: String,
  pub color: String,
  pub emoji: String,
  pub owner_id: String,
  pub items: Vec<Item>,
  pub created_at: String
}


#[cfg(test)]
mod whishlists {
  use super::*;

  #[test]
  fn ut_test_collection() {
    let items: Vec<Item> = Vec::new();

    let c = Collection {
      id: String::from("14c97177-33c4-4e41-8af6-94241526a7c4"),
      name: String::from("Libros"),
      color: String::from("#eaa510"),
      emoji: String::from("📖"),
      items: items,
      owner_id: String::from("14c97177-33c4-4e41-8af6-94241526a7c4"),
      created_at: String::from("2022-04-18T22:39:16.627Z")
    };

    assert_eq!(c.name, String::from("Libros"))
  }
}