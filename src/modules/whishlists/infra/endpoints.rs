use rocket::{get, State};

use crate::infra::state::Repositories;
use crate::modules::whishlists::app::collection;

#[get("/")]
pub fn get_collections(state: &State<Repositories>) -> String {
  let manager  = collection::Manager{
    repo: Box::new(state.collection.clone())
  };

  match manager.list() {
    Ok(l) => { println!("{:?}", l); },
    Err(e) => { println!("eee") }
  };

  format!("Token for ")
} 