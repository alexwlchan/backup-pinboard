use std::fs::File;
use std::io::Write;
use std::process;

extern crate docopt;
extern crate hyper;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;

mod assets;
mod cli;
mod metadata;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

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

        let cache_ids = assets::get_cache_ids(username, password);
        println!("{:?}", cache_ids);
        eprintln!("Archive backups are not implemented yet!");
        process::exit(2);
    }
}
