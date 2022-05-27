use regex::Regex;
use serde::{Deserialize};
use validator::{Validate};


lazy_static! {
  static ref PRICE_REGEX: Regex = Regex::new(r"[0-9]{1,9}$").unwrap();
}


/*
 * Item data transfer object
*/

#[derive(Deserialize, Validate, Clone)]
pub struct CreateItem {
  #[validate(length(min = 1))]
  pub title: String,
  #[validate(url)]
  pub image: String,
  #[validate(url)]
  pub website: String,
  #[validate(regex = "PRICE_REGEX")]
  pub price: String
}

#[derive(Deserialize, Validate, Clone)]
pub struct UpdateItem {
  #[validate(length(min = 1))]
  pub title: Option<String>,
  #[validate(url)]
  pub image: Option<String>,
  #[validate(url)]
  pub website: Option<String>,
  #[validate(regex = "PRICE_REGEX")]
  pub price: Option<String>,
  pub obtained: Option<bool>
}

#[cfg(test)]
mod whishlists {
  use super::*;

  #[test]
  fn ut_validate_create_item_dto() {
    let item = CreateItem {
      title: String::from("Ropa"),
      image: String::from("https://m.media-amazon.com/images/P/B019H695T4.01._SCLZZZZZZZ_SX500_.jpg"),
      website: String::from("https://www.amazon.com.mx/El-hombre-en-busca-sentido/dp/8425432022/ref=sr_1_1?adgrpid=1161084699107046&hvadid=72567867197970&hvbmt=be&hvdev=c&hvlocphy=119&hvnetw=s&hvqmt=e&hvtargid=kwd-72568145275092%3Aloc-119&hydadcr=27013_11422070&keywords=el+hombre+en+busca+de+sentido&qid=1652998524&sr=8-1"),
      price: String::from("340.00")
    };

    match item.validate() {
      Ok(_) => {
        assert_eq!(item.price, String::from("340.00"))
      },
      Err(e) => {
        panic!("Validation err {}", e)
      }
    }
  }

  #[test]
  fn ut_validate_update_item_dto() {
    let item = UpdateItem {
      title: Some(String::from("Aloja")),
      image: None,
      website: None,
      price: None,
      obtained: None
    };

    match item.validate() {
      Ok(_) => {
        assert_eq!(item.title, Some(String::from("Aloja")))
      },
      Err(e) => {
        panic!("Validation err {}", e)
      }
    }
  }
}
