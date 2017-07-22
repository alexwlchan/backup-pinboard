use std::io::Read;
use std::process;

use reqwest;


/// Return metadata about all your posts from the Pinboard API.
///
///  - `username`: Pinboard username
///  - `password`: Pinboard password
///
pub fn get_metadata(username: String, password: String) -> String {

  let client = reqwest::Client::new().unwrap();
  let resp = client.get("https://api.pinboard.in/v1/posts/all").unwrap()
    .basic_auth(username, Some(password))
    .send();
  let mut content = String::new();

  match resp {
    Ok(mut r) => {
      if r.status().is_success() {
        let _ = r.read_to_string(&mut content);
      } else {
        eprint!("Error status code from the Pinboard API: {}", r.status());
        process::exit(1);
      }
    },
    Err(e) => {
      eprint!("Unexpected error calling the Pinboard API: {}", e);
      process::exit(2);
    },
  }

  content
}