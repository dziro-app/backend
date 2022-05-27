use serde::{Serialize, Deserialize};

/*
 * Item
{
  "id": "e37f5ca9-fba1-46d9-a3ef-e80ada650784",
  "title": "El hombre en busca de sentido : Frankl, Viktor E.: Amazon.com.mx: Libros",
  "website": "https://www.amazon.com.mx/El-hombre-en-busca-sentido/dp/8425432022/ref=sr_1_1?adgrpid=1161084699107046&hvadid=72567867197970&hvbmt=be&hvdev=c&hvlocphy=119&hvnetw=s&hvqmt=e&hvtargid=kwd-72568145275092%3Aloc-119&hydadcr=27013_11422070&keywords=el+hombre+en+busca+de+sentido&qid=1652998524&sr=8-1",
  "price": 221,
  "image": "https://m.media-amazon.com/images/P/B019H695T4.01._SCLZZZZZZZ_SX500_.jpg",
  "obtained": false,
  "created_at": "2022-05-19T22:46:33.075Z"
}
*/

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
  pub id: String,
  pub title: String,
  pub website: String,
  pub price: f32,
  pub image: String,
  pub obtained: bool,
  pub created_at: String
}



#[cfg(test)]
mod whishlists {
  use super::*;

  #[test]
  fn ut_test_item_entity() {
    let i = Item {
      id: String::from("e37f5ca9-fba1-46d9-a3ef-e80ada650784"),
      title: String::from("El hombre en busca de sentido : Frankl, Viktor E.: Amazon.com.mx: Libros"),
      website: String::from("https://www.amazon.com.mx/El-hombre-en-busca-sentido/dp/8425432022/ref=sr_1_1?adgrpid=1161084699107046&hvadid=72567867197970&hvbmt=be&hvdev=c&hvlocphy=119&hvnetw=s&hvqmt=e&hvtargid=kwd-72568145275092%3Aloc-119&hydadcr=27013_11422070&keywords=el+hombre+en+busca+de+sentido&qid=1652998524&sr=8-1"),
      price: 221.3,
      image: String::from("https://m.media-amazon.com/images/P/B019H695T4.01._SCLZZZZZZZ_SX500_.jpg"),
      obtained: false,
      created_at: String::from("2022-05-19T22:46:33.075Z")
    };

    assert_eq!(i.price, 221.3);
  }
}