use std::collections::HashMap;
use std::io::Read;
use std::process;

use hyper::header::Header;
use regex::Regex;
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


/// Given a blob of HTML from a Pinboard index, update the map of cache IDs
/// and corresponding links.
fn get_cache_ids_from_html(html: &str, cache_ids: &mut HashMap<String, String>) {
    // Parsing HTML with regex or string manipulation is, in general,
    // a terrible idea.  I'm doing it here because I couldn't work out how
    // to use html5ever from their API docs, and I'm tired.
    // TODO: Write this to be not terrible.

    // All the bookmarks on a page are in a
    //
    //      <div id="bookmarks"> … </div>
    //
    // block, so start by extracting that.
    let bookmarks_div = html
        .split("<div id=\"bookmarks\">").collect::<Vec<&str>>()[1]
        .split("<div id=\"right_bar\">").collect::<Vec<&str>>()[0];

    // Individual bookmarks always have
    //
    //      <div name="edit_checkbox" class="edit_checkbox>">
    //
    // at the top, so we can use this as a rough proxy for bookmarks.
    let mut bookmarks = bookmarks_div
        .split("<div name=\"edit_checkbox\" class=\"edit_checkbox\">");

    // Discard the first entry.
    bookmarks.next();

    // The links to cached bookmarks are of the form
    //
    //      <a class="cached" href="/cached/123456789abcdef/">☑</a>
    //
    let cached_re = Regex::new(
        "<a class=\"cached\" href=\"/cached/(?P<cache_id>[0-9a-f]+)/\">"
    ).unwrap();

    // The links to bookmarks are of the form
    //
    //      <a class="bookmark_title" href="..."
    //
    let link_re = Regex::new(
        "<a class=\"bookmark_title[a-z_ ]+\"\\s*href=\"(?P<link_href>[^\"]+)\""
    ).unwrap();

    for b in bookmarks {
        if cached_re.is_match(b) {
            let cache_match = cached_re.captures(b)
                .unwrap()["cache_id"]
                .to_owned();

            let link_match = link_re.captures(b)
                .unwrap()["link_href"]
                .to_owned();

            cache_ids.insert(link_match, cache_match);
        } else {
            // This doesn't have a link, so we can't save it.
            continue;
        }
    }
}


fn get_html_for_page(client: &Client, path: &str, cookie: &Cookie) -> String {
    let url = format!("https://pinboard.in{}", path);
    let resp = client.get(&url)
        .unwrap()
        .header(cookie.to_owned())
        .send();
    let mut content = String::new();
    let _ = resp.ok().unwrap().read_to_string(&mut content);
    content
}


fn get_next_page_path(html: &str) -> Option<String> {
    let next_href_re = Regex::new(
        "<a class=\"next_prev\" href=\"(?P<next_href>[^\"]+)\">« earlier"
    ).unwrap();

    match next_href_re.captures(html) {
        Some(s) => Some(s["next_href"].to_owned()),
        None => None,
    }
}


fn update_cache_ids_for_path(client: &Client, path: &str, cookie: &Cookie, mut cache_ids: &mut HashMap<String, String>) {
    println!("Inspecting path {}", path);
    let content = get_html_for_page(&client, &path, &cookie);
    get_cache_ids_from_html(&content, &mut cache_ids);
    match get_next_page_path(&content) {
        Some(next_path) => update_cache_ids_for_path(&client, &next_path, &cookie, &mut cache_ids),
        None => {}
    };
}


/// Return a map from URLs to Pinboard cache IDs.
///
///  - `username`: Pinboard username
///  - `password`: Pinboard password
///
pub fn get_cache_ids(username: &str, password: &str) -> HashMap<String, String> {

    let client = Client::new().unwrap();
    let cookie = get_login_cookie(&username, &password);
    let mut cache_ids = HashMap::new();

    // Now fetch a blob of HTML for the first page.
    let path = format!("/u:{}/", username);
    update_cache_ids_for_path(&client, &path, &cookie, &mut cache_ids);

    cache_ids
}
