use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::str;

use docopt::Docopt;

use Manifest;
use utils::{run, run_filter};

#[cfg_attr(rustfmt, rustfmt_skip)]
const USAGE: &'static str = "
usage: travis_cargo.py doc-upload [-h] [--branch BRANCH]

Use ghp-import to upload cargo-rendered docs to Github Pages, from the master
branch.

optional arguments:
  -h, --help       show this help message and exit
  --branch BRANCH  upload docs when on this branch, defaults to master";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_branch: Option<String>,
}

pub fn doc_upload(manifest: Manifest, args: env::Args) {
    let args: Args = Docopt::new(USAGE)
                         .and_then(|dopt| dopt.argv(args.into_iter()).decode())
                         .unwrap_or_else(|e| e.exit());

    let branch = env::var("APPVEYOR_REPO_BRANCH").or(env::var("TRAVIS_BRANCH")).unwrap();
    let repo = env::var("APPVEYOR_REPO_NAME").or(env::var("TRAVIS_REPO_SLUG")).unwrap();
    let pr = if env::var("APPVEYOR_PULL_REQUEST_NUMBER").is_ok() {
        true
    } else {
        env::var("TRAVIS_PULL_REQUEST").map(|pr| pr.parse::<bool>().unwrap()).unwrap()
    };

    let lib_name = manifest.lib_name().unwrap();
    if branch == args.flag_branch.unwrap_or("master".to_owned()) && !pr {
        // only load the token when we're sure we're uploading (travis
        // won't decrypt secret keys for PRs, so loading this with the
        // other vars causes problems with tests)
        let token = env::var("GH_TOKEN").unwrap();
        println!("uploading docs...");
        let mut file = File::open("target/doc/index.html").unwrap();
        writeln!(file,
                 "<meta http-equiv=refresh content=0;url={}/index.html>",
                 &lib_name)
            .unwrap();

        run(Command::new("git").args(&["clone", "https://github.com/davisp/ghp-import"]));
        run(Command::new("python").args(&["./ghp-import/ghp-import", "-n", "target/doc"]));
        let repo_url = format!("https://{}@github.com/{}.git", token, repo);
        run_filter(&token,
                   Command::new("git").args(&["push", "-fq", &repo_url, &branch]));
    }
}
