use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process;

extern crate docopt;
extern crate hyper;
extern crate regex;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;

mod assets;
mod cli;
mod metadata;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");


fn create_wget_login_cookie(username: &str, password: &str) {
    process::Command::new("wget")
        .args(&["--save-cookies", "/tmp/pinboard-cookies.txt"])
        .arg("--keep-session-cookies")
        .arg("--delete-after")
        .args(&["--output-file", "-"])
        .args(&["--post-data", &format!("username={}&password={}", username, password)])
        .arg("https://pinboard.in/auth/")
        .output()
        .expect("Failed to authenticate against Pinboard with wget");
}

fn main() {
    let args = cli::parse_args(NAME);

    if args.flag_version {
        println!("{} v{}", NAME, VERSION);
    }

    else if args.cmd_metadata {
        let username = args.flag_username.unwrap();
        let password = args.flag_password.unwrap();
        let outfile = args.flag_outfile.unwrap_or("bookmarks.json".to_owned());
        let format = metadata::guess_format(&outfile);

        let data = metadata::get_metadata(username, password, format);

        let mut buffer = File::create(&outfile).unwrap();
        let _ = buffer.write(data.as_bytes());
    }

    else if args.cmd_archive {
        let username = args.flag_username.unwrap();
        let password = args.flag_password.unwrap();
        let out_dir = args.flag_outdir.unwrap_or("archive".to_owned());

        let cache_ids = assets::get_cache_ids(&username, &password);
        println!("{:?}", cache_ids);

        create_wget_login_cookie(&username, &password);

        for cache_id in cache_ids.values() {
            println!("Fetching cache for {}", cache_id);
            let url = format!("https://pinboard.in/cached/{}/", cache_id);
            let local_out_dir = format!("{}/{}", out_dir, cache_id);
            let path = Path::new(&local_out_dir);
            if path.exists() {
                println!("Cache for {} already exists, skipping", cache_id);
                continue;
            }
            process::Command::new("wget")
                .arg("--adjust-extension")
                .arg("--span-hosts")
                .arg("--no-verbose")
                .arg("--convert-links")
                .arg("--page-requisites")
                .arg("--no-directories")
                .args(&["-e", "robots=off"])
                .args(&["--load-cookies", "/tmp/pinboard-cookies.txt"])
                .args(&["--output-file", "-"])
                .args(&["--directory-prefix", &local_out_dir])
                .arg(&url)
                .output()
                .expect("Failed to download archive from Pinboard");
        }
    }
}
