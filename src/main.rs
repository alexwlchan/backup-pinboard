use std::fs::File;
use std::io::Write;
use std::process;

extern crate docopt;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;

mod cli;
mod metadata;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = cli::parse_args(NAME);

    if args.flag_version {
        println!("{} v{}", NAME, VERSION);
    }

    if args.cmd_metadata {
        let username = args.flag_username.unwrap();
        let password = args.flag_password.unwrap();
        let outfile = args.flag_outfile.unwrap_or("bookmarks.json".to_owned());

        let data = metadata::get_metadata(username, password);

        let mut buffer = File::create(&outfile).unwrap();
        let _ = buffer.write(data.as_bytes());
    }

    if args.cmd_archive {
        eprintln!("Archive backups are not implemented yet!");
        process::exit(2);
    }
}
