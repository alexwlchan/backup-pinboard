use std::collections::HashMap;
use std::process;

use hyper::header::Location;
use reqwest::{Client, RedirectPolicy};


/// Return a map from URLs to Pinboard cache IDs.
///
///  - `username`: Pinboard username
///  - `password`: Pinboard password
///
pub fn get_cache_ids(username: String, password: String) -> HashMap<String, String> {

    // Start by logging in to Pinboard, so we have the appropriate cookies.
    // Because Pinboard sends us into a weird redirect loop, we have to
    // tell reqwest not to follow redirects, just check the redirect worked.
    let client = Client::builder()
        .unwrap()
        .redirect(RedirectPolicy::none())
        .build()
        .unwrap();
    println!("foo");
    let resp = client.post("https://pinboard.in/auth/")
        .unwrap()
        .form(&[("username", username), ("password", password)])
        .unwrap()
        .send();
    let login_successful = resp.ok()
        .unwrap()
        .headers()
        .get::<Location>()
        .unwrap() != &Location::new("?error=wrong+password");
    if !login_successful {
        eprintln!("Error logging in to Pinboard!");
        process::exit(1);
    }

    println!("{:?}", login_successful);

    HashMap::new()
}
