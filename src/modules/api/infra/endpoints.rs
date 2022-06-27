use rocket::{get, http::{Status}, response::{content, status}};


#[get("/version")]
pub fn get_version() ->  status::Custom<content::RawJson<String>>  {
 
  let content = format!("{{ \"version\": \"0.0.1\" }}");
  return status::Custom(Status::Ok, content::RawJson(content));
}