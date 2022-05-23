use serde::{Deserialize};


/*
 * Collection 
*/

#[derive(Deserialize, Clone)]
pub struct CreateCollection {
  pub name: String,
  pub color: String,
  pub emoji: String
}

#[derive(Deserialize, Clone)]
pub struct UpdateCollection {
  pub name: Option<String>,
  pub color: Option<String>,
  pub emoji: Option<String>
}


/*
 * Item 
*/

#[derive(Deserialize, Clone)]
pub struct CreateItem {
  pub title: String,
  pub image: String,
  pub website: String,
  pub price: String
}


#[derive(Deserialize, Clone)]
pub struct UpdateItem {
  pub title: Option<String>,
  pub image: Option<String>,
  pub website: Option<String>,
  pub price: Option<u8>
}
