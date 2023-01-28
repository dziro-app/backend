use serde::{Serialize, Deserialize};

/*
 * Collaborator 
 */

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Collaborator {
  pub user_id: String,
  pub can_edit: bool,
  pub active: bool
}