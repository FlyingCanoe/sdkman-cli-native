use std::fs;
use std::fs::remove_dir_all;
use std::process;

use clap::Parser;
use colored::Colorize;
use symlink::remove_symlink_dir;

use sdkman_cli_native::constants::{CANDIDATES_DIR, CURRENT_DIR};
use sdkman_cli_native::helpers::{
    infer_candidate_version, infer_sdkman_dir, known_candidates, validate_candidate,
    validate_version_path,
};

#[derive(Parser, Debug)]
#[command(
    bin_name = "sdk install",
    about = "sdk subcommand to install a candidate version"
)]
struct Args {
    #[arg(required(true))]
    candidate: String,
    version: Option<String>,
    folder: Option<String>,
}

fn main() {
    let args = Args::parse();
    let candidate = args.candidate;
    let version = args.version;
    let folder = args.folder;

    let version = infer_candidate_version(candidate.as_str(), version, folder);
    /*let force = args.force;
    let sdkman_dir = infer_sdkman_dir();

    let candidate = validate_candidate(known_candidates(sdkman_dir.to_owned()), &candidate);

    let candidate_path = sdkman_dir.join(CANDIDATES_DIR).join(&candidate);
    let version_path = validate_version_path(sdkman_dir, &candidate, &version);
    let current_link_path = candidate_path.join(CURRENT_DIR);
    if current_link_path.is_dir() {
        match fs::read_link(current_link_path.to_owned()) {
            Ok(relative_resolved_dir) => {
                let resolved_link_path = candidate_path.join(relative_resolved_dir);
                if (version_path == resolved_link_path) && force {
                    remove_symlink_dir(&current_link_path).unwrap_or_else(|_| {
                        remove_dir_all(current_link_path.to_owned()).expect(&format!(
                            "cannot remove current directory for {}.",
                            candidate
                        ))
                    });
                } else if (version_path == resolved_link_path) && !force {
                    eprintln!(
                        "\n{} {} is the {} version and should not be removed.",
                        candidate.bold(),
                        version.bold(),
                        "current".italic(),
                    );
                    println!(
                        "\n\nOverride with {}, but leaves the candidate unusable!",
                        "--force".italic()
                    );
                    process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("current link broken, stepping over: {}", e.to_string());
            }
        }
    }

    remove_dir_all(version_path)
        .map(|_| {
            println!("removed {} {}.", candidate.bold(), version.bold());
        })
        .expect("panic! could not delete directory.");*/
}
