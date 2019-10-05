use clap::{App, Arg, ArgMatches};
use regex::RegexBuilder;

use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;

fn main() {
    let args = get_args();

    let mut match_regex = String::from(args.value_of("match_regex").unwrap());
    let rename_regex = args.value_of("rename_regex").unwrap();

    let dry_run = args.is_present("dry_run");
    let full_match = args.is_present("full_match");
    let ignore_case = args.is_present("ignore_case");
    let prompt = args.is_present("prompt");

    if full_match {
        match_regex = format!("^{}$", match_regex);
    }

    let mut regex_builder = RegexBuilder::new(match_regex.as_str());
    regex_builder.case_insensitive(ignore_case);

    let re = regex_builder.build().unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1);
    });

    let files: Vec<&str> = args.values_of("files").unwrap().collect();

    let mut new_files = Vec::with_capacity(files.len());
    let mut hs = std::collections::HashSet::with_capacity(files.len());

    // Create a new list of all the renamed files
    for file in &files {
        let new_file = re.replace_all(file, rename_regex).into_owned();
        new_files.push(new_file);
    }

    // Check for collisions in the renamed file list
    for new_file in &new_files {
        hs.insert(new_file);
    }

    if hs.len() < new_files.len() {
        eprintln!("ERROR: Collision exists in new file names. Aborting...");
        exit(1);
    }

    for (file, new_file) in files.iter().zip(&new_files) {
        if file == new_file {
            continue;
        }

        println!("{} => {}", file, new_file);

        if !dry_run {
            let path = Path::new(new_file);

            if prompt && path.exists() {
                print!("overwrite {}? (y/n [n]) ", new_file);
                io::stdout().flush().unwrap();

                let response = read_from_stdin().to_ascii_lowercase();
                let response = response.trim();

                if response == "y" {
                    rename(file, new_file);
                }
            } else {
                rename(file, new_file);
            }
        }
    }

    exit(0);
}

fn get_args<'a>() -> ArgMatches<'a> {
    App::new("mvr")
        .version("0.1")
        .author("Collin Styles <collingstyles@gmail.com")
        .about("The mv coreutil but with regexes")
        .arg(Arg::with_name("match_regex").index(1).required(true))
        .arg(Arg::with_name("rename_regex").index(2).required(true))
        .arg(Arg::with_name("files").multiple(true).required(true))
        .arg(
            Arg::with_name("dry_run")
                .short("n")
                .long("dry-run")
                .help("Print changes but don't actually rename any files"),
        )
        .arg(
            Arg::with_name("full_match")
                .short("m")
                .long("full-match")
                .help("Only rename a file if its filepath is fully matched"),
        )
        .arg(
            Arg::with_name("ignore_case")
                .short("c")
                .long("ignore-case")
                .help("Search case-insensitively"),
        )
        .arg(
            Arg::with_name("prompt")
                .short("i")
                .long("prompt")
                .help("Prompt before overwriting an existing file"),
        )
        .get_matches()
}

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    buffer
}

fn rename(old_file: &str, new_file: &str) {
    if let Err(err) = std::fs::rename(old_file, new_file.to_string()) {
        eprintln!("ERROR: {}", err);
        exit(1);
    }
}
