use docopt::Docopt;

const USAGE: &str = "
Usage: <NAME> metadata --username=<USERNAME> --password=<PASSWORD> [--outfile=<OUTFILE>]
       <NAME> archive --username=<USERNAME> --password=<PASSWORD> [--outdir=<OUTDIR>]
       <NAME> (-h | --help)
       <NAME> --version

Options:
  -h --help               Show this screen.
  --version               Show version.
  --username=<USERNAME>   Pinboard username.
  --password=<PASSWORD>   Pinboard password.
  --outfile=<OUTFILE>     Write your bookmark metadata to OUTFILE.
  --outdir=<OUTDIR>       Save your archived bookmarks in OUTDIR.

Commands:
  metadata                Save a JSON file containing metadata about all your
                          bookmarks.
  archive                 Download a copy of the bookmarks saved by the
                          Pinboard archiver (https://pinboard.in/upgrade/).
                          Requires GNU wget.
";

#[derive(Debug, Deserialize)]
pub struct Args {
  pub cmd_metadata: bool,
  pub cmd_archive: bool,
  pub flag_version: bool,
  pub flag_username: Option<String>,
  pub flag_password: Option<String>,
  pub flag_outfile: Option<String>,
  pub flag_outdir: Option<String>,
}

pub fn parse_args(name: &str) -> Args {
  Docopt::new(str::replace(USAGE, "<NAME>", name))
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit())
}
