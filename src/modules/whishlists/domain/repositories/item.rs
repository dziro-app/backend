use crate::modules::whishlists::{
  domain::entities::item::Item,
  dtos::item::UpdateItem,
};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg(test)]
use crate::modules::whishlists::{
  dtos::item::CreateItem,
  app::item
};

#[cfg_attr(test, automock)]
pub trait ItemRepository {
  fn find(&self, user_id: String, id: String) -> Result<Item, String>;
  fn save(&self, user_id: String, collection_id: String, data: Item) -> Result<Item, String>;
  fn update(&self, user_id: String, id: String, data: UpdateItem) -> Result<Item, String>;
  fn delete(&self, user_id: String, id: String) -> Result<(), String>;
}


#[cfg(test)]
mod item_repository {
  use super::*;

  fn get_mock_item() -> Item {
    return Item { 
      id: String::from(""),
      title: String::from("Ropa"), 
      website: String::from(""), 
      price: 34.40, 
      image: String::from(""), 
      obtained: false, 
      created_at: String::from("") 
    }
  }

  #[test]
  fn ut_create_an_item(){
    let mut repo = MockItemRepository::new();

    repo.expect_save()
      .once()
      .returning(| _user_id, _collection_id, _data| Ok(get_mock_item()));
  
    let manager = item::Manager {
      repo: Box::new(repo),
      user_id: String::from("222f731a-4a70-4e6b-acd5-d233c9f80a98")
    };

    let data = CreateItem {
      title: String::from("Ropa"),
      image: String::from("https://m.media-amazon.com/images/P/B019H695T4.01._SCLZZZZZZZ_SX500_.jpg"),
      website: String::from("https://www.amazon.com.mx/El-hombre-en-busca-sentido/dp/8425432022/ref=sr_1_1?adgrpid=1161084699107046&hvadid=72567867197970&hvbmt=be&hvdev=c&hvlocphy=119&hvnetw=s&hvqmt=e&hvtargid=kwd-72568145275092%3Aloc-119&hydadcr=27013_11422070&keywords=el+hombre+en+busca+de+sentido&qid=1652998524&sr=8-1"),
      price: String::from("300.00")
    };

    match manager.create(String::from("00a3313b-c3a4-4bbf-ae94-8434a3bed088"), data) {
      Ok(c) => {
        println!("{:#?}", c);
        assert_eq!(c.title, String::from("Ropa"));
      },
      Err(e) => {
        panic!("{}", e);
      }
    };

  }

  #[test]
  fn ut_delete_an_item(){
    let mut repo = MockItemRepository::new();

    repo.expect_delete()
      .once()
      .returning(| _user_id, _collection_id | Ok(()));
  
    let manager = item::Manager {
      repo: Box::new(repo),
      user_id: String::from("222f731a-4a70-4e6b-acd5-d233c9f80a98")
    };

    manager.delete(String::from("654e0771-fa7f-4108-a326-ace4f72a39df")).unwrap();
  }


  #[test]
  fn ut_updates_an_item(){
    let mut repo = MockItemRepository::new();

    repo.expect_update()
      .once()
      .returning(| _user_id, _collection_id, _data | Ok(get_mock_item()));
  
    let manager = item::Manager {
      repo: Box::new(repo),
      user_id: String::from("222f731a-4a70-4e6b-acd5-d233c9f80a98")
    };

    let data = UpdateItem {
      image: None,
      title: Some(String::from("El hombre en busca de sentido")),
      website: None,
      price: None,
      obtained: Some(true)
    };

    manager.update(String::from("654e0771-fa7f-4108-a326-ace4f72a39df"), data).unwrap();
  }
}