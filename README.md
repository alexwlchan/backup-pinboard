# backup-pinboard

This is a tool for creating a local backup of your Pinboard bookmarks.

## Installation

Building this project [requires Rust][rust] (1.19 or later).
Then install the project using Cargo:

```console
$ cargo install --git https://github.com/alexwlchan/backup-pinboard.git
```

You need to add `~/.cargo/bin` to your PATH.

[rust]: https://www.rust-lang.org/en-US/

## Usage

Download your bookmarks metadata:

```console
$ backup-pinboard metadata --username=USERNAME --password=PASSWORD
```

This downloads your metadata to `bookmarks.json`.
You can specify an alternative path with `--outfile`, for example:

```console
$ backup-pinboard metadata --username=USERNAME --password=PASSWORD --outfile=~/backups/pinboard.json
```

If the outfile name ends in `.xml`, metadata is saved as XML instead of JSON.

If you have an archival account, you can also download copies of your saved pages:

```console
$ backup-pinboard archive --username=USERNAME --password=PASSWORD --outdir=~/backups/pinboard-archive
```

## The old Python script

This used to be a Python script; in July 2017 I completely rewrote the tool in Rust.
You can browse the repo [at commit 6dcc06e][python] for the last version of the Python scripts.

[python]: https://github.com/alexwlchan/backup-pinboard/tree/6dcc06e49a4863803d5a2a0c9ac23bfec2f4bcf3

## License

MIT.
