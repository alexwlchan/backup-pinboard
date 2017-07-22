use std::collections::HashMap;
use std::io::Read;
use std::process;

use hyper::header::Header;
use reqwest::header::{Cookie, Location, SetCookie};
use reqwest::{Client, RedirectPolicy};


/// Log in to Pinboard and get a login cookie that can be used on subsequent
/// requests.
///
fn get_login_cookie(username: &str, password: &str) -> Cookie {
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
        .form(&[("username", &username), ("password", &password)])
        .unwrap()
        .send();
    let headers = resp.ok().unwrap().headers().to_owned();
    let login_successful = headers
        .get::<Location>()
        .unwrap() != &Location::new("?error=wrong+password");
    if !login_successful {
        eprintln!("Error logging in to Pinboard!");
        process::exit(1);
    }

    // Cookie-handling code.  This should probably be handled by reqwest,
    // but I've given up trying to get it working, and I'm just doing it
    // by hand instead.  Pinboard only sets four interesting cookie values.
    let set_cookie = headers
        .get::<SetCookie>()
        .unwrap();
    let mut cookie = Cookie::new();
    for c in set_cookie.iter() {
        let parsed = Cookie::parse_header(&c.as_bytes().into()).unwrap();
        for key in vec!["groznaz", "auth", "secauth", "login"].iter() {
            match parsed.get(key) {
                Some(val) => cookie.set(key.to_owned(), val.to_owned()),
                None => {},
            };
        }
    }

    cookie
}


/// Return a map from URLs to Pinboard cache IDs.
///
///  - `username`: Pinboard username
///  - `password`: Pinboard password
///
pub fn get_cache_ids(username: String, password: String) -> HashMap<String, String> {

    let client = Client::new().unwrap();
    let cookie = get_login_cookie(&username, &password);

    // Now fetch a blob of HTML for the first page.
    let url = format!("https://pinboard.in/u:{}/", username);
    let resp = client.get(&url)
        .unwrap()
        .header(cookie)
        .send();
    let mut content = String::new();
    let _ = resp.ok().unwrap().read_to_string(&mut content);

    // println!("{:?}", content);

    HashMap::new()
}
