# handle_release_rs
CLI tool for managing releases in Rust repositories.

## Requirements
- Linux OS (Ubuntu 24.04.2)
- Rust >= 1.90.0

## Install
The easiest way to get started is to download a precompiled Linux binary from the latest [release](https://github.com/OscarAspelin95/handle_release_rs/releases).

## Usage
The script will automatically:
- Check that current branch is `main`.
- Check that worktree is clean.
- Fetch the latest tag (requires semantic versioning).
- Increment and push the new tag.

`handle_release_rs --directory <directory> --bump-type <bump-type>`

Required arguments:
<pre>
<b>--directory</b> - Path to directory with manifest file (`Cargo.toml`).
<b>--bump-type</b> - Either `major`, `minor` or `patch`.
</pre>

Optional arguments:
<pre>
<b>--dry-run</b> - Run all checks, but don't create and push a tag.
</pre>


## Roadmap
- Add support for providing multiple accessions.
- Add filtering options for e.g., platform, fastq byte size, etc.
- Streaming support for large files.
- Add dynamic timeout based on fastq byte size.
