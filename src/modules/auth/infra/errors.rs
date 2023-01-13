use std::result;

#[derive(Debug)]
pub enum OauthError {
  Network(String),
  Authorization(String),
  MismatchingResponse(String)
}

pub type OauthResult<T> = result::Result<T, OauthError>;
