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
    println!("{}", data);
  }
}
