#![deny(warnings)]

extern crate docopt;
#[macro_use]
extern crate serde_derive;

mod cli;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
  let args = cli::parse_args(NAME);

  if args.flag_version {
    println!("{} v{}", NAME, VERSION);
  }

  println!("{:?}", args);
}
