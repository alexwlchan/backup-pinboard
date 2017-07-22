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
    let data = metadata::get_metadata(args.flag_username.unwrap(), args.flag_password.unwrap());

    let mut buffer = File::create("bookmarks.xml").unwrap();
    let _ = buffer.write(data.as_bytes());
  }

  if args.cmd_archive {
    eprintln!("Archive backups are not implemented yet!");
    process::exit(2);
  }
}
