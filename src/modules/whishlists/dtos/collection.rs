use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate};

lazy_static! {
  static ref COLOR_REGEX: Regex = Regex::new(r"#[0-9a-fA-F]{6,6}$").unwrap();
}

/*
 * Collection data transfer object
*/

#[derive(Deserialize, Serialize, Validate, Clone, Debug)]
// #[serde(crate = "rocket::serde")]
pub struct CreateCollection {
  #[validate(length(min = 1))]
  pub name: String,
  #[validate(regex = "COLOR_REGEX")]
  pub color: String,
  #[validate(length(min = 1))]
  pub emoji: String
}

#[derive(Deserialize, Serialize, Validate, Clone)]
pub struct UpdateCollection {
  #[validate(length(min = 1))]
  pub name: Option<String>,
  #[validate(regex = "COLOR_REGEX")]
  pub color: Option<String>,
  #[validate(length(min = 1))]
  pub emoji: Option<String>
}


#[cfg(test)]
mod whishlists {
  use super::*;

  #[test]
  fn ut_validate_create_collection_dto() {
    let collection = CreateCollection {
      name: String::from("Ropa"),
      color: String::from("#00AF00"),
      emoji: String::from("⏰")
    };

    match collection.validate() {
      Ok(_) => {
        assert_eq!(collection.name, String::from("Ropa"))
      },
      Err(e) => {
        panic!("Validation err {}", e)
      }
    }
  }

  #[test]

  fn ut_validate_update_collection_dto() {
    let collection = UpdateCollection {
      name: Some(String::from("Ropa")),
      color: Some(String::from("#00AF00")),
      emoji: None
    };

    match collection.validate() {
      Ok(_) => {
        assert_eq!(collection.emoji, None);
        assert_eq!(collection.name, Some(String::from("Ropa")));
      },
      Err(e) => {
        panic!("Validation err {}", e)
      }
    }
  }
}