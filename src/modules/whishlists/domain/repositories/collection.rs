use crate::modules::whishlists::{
  domain::entities::collection::Collection,
  dtos::collection::{UpdateCollection}
};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg(test)]
use crate::modules::whishlists::{
  dtos::collection::CreateCollection,
  app::collection::Manager
};

#[cfg_attr(test, automock)]
pub trait CollectionRepository {
  fn list(&self, user_id: String) -> Result<Vec<Collection>, String>;
  fn save(&self, data: Collection) -> Result<Collection, String>;
  fn update(&self, user_id: String, id: String, data: UpdateCollection) -> Result<Collection, String>;
  fn delete(&self, user_id: String, id: String) -> Result<(), String>;
}


#[cfg(test)]
mod collection_repository {
  use super::*;

  #[test]
  fn ut_create_a_collection() {
    let mut repo = MockCollectionRepository::new();

    repo.expect_save()
      .once()
      .returning(|x| Ok(x));
    
    let manager = Manager {
      repo: Box::new(repo),
      user_id: String::from("222f731a-4a70-4e6b-acd5-d233c9f80a98")
    };
  
    let data = CreateCollection {
      name: String::from("Disfraces"),
      color: String::from("#00AF00"),
      emoji: String::from("⏰")
    };

    match manager.create(data) {
      Ok(c) => {
        println!("{:#?}", c);
        assert_eq!(c.name, String::from("Disfraces"));
      },
      Err(e) => {
        panic!("{}", e);
      }
    };

  }

  #[test]
  fn ut_delete_a_collection() {
    let mut repo = MockCollectionRepository::new();

    repo.expect_delete()
      .once()
      .returning(|_user_id, _id| Ok(()));
    
    let manager = Manager {
      repo: Box::new(repo),
      user_id: String::from("222f731a-4a70-4e6b-acd5-d233c9f80a98")
    };

    match manager.delete(String::from("")) {
      Ok(_) => {},
      Err(e) => {
        panic!("{}", e);
      }
    };
  }

  #[test]
  fn ut_list_collections() {
    let mut repo = MockCollectionRepository::new();

    repo.expect_list()
      .once()
      .returning(|_user_id| Ok( vec![] ));
    
      let manager = Manager {
      repo: Box::new(repo),
      user_id: String::from("222f731a-4a70-4e6b-acd5-d233c9f80a98")
    };

    match manager.list() {
      Ok(cs) => {
        println!("{:#?}", cs);
      }, 
      Err(e) => {
        panic!("{}", e);
      }
    };
  }

  #[test]
  fn ut_update_a_collection() {
    let mut repo = MockCollectionRepository::new();

    repo.expect_update()
      .once()
      .returning(|_user_id, _id, _data| Ok( Collection { 
        id: String::from("222f731a-4a70-4e6b-acd5-d233c9f80a99"), 
        name: String::from("Ropa6"), 
        color: String::from("#00ff89"), 
        emoji: String::from("😎"), 
        owner_id: String::from("222f731a-4a70-4e6b-acd5-d233c9f80a98"), 
        items: vec![], 
        created_at: String::from("")
      } ));
    
    let manager = Manager {
      repo: Box::new(repo),
      user_id: String::from("222f731a-4a70-4e6b-acd5-d233c9f80a98")
    };

    let new_data = UpdateCollection {
      name: Some(String::from("Ropa6")),
      color: Some(String::from("#00ff89")),
      emoji: Some(String::from("😎"))
    };

    let n = manager.update(String::from(""), new_data).unwrap();
    assert_eq!(n.name, String::from("Ropa6"));
  }
}