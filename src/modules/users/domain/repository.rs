use crate::modules::users::domain::entities::User;

pub trait UserRepositoy {
  fn save(&self, user: User) -> Result<User, String>;
  fn find(&self, id: String) -> Option<User>;
}