extern crate clap;
extern crate regex;

use clap::{Arg, App, ArgMatches};
use regex::Regex;

fn main() {
    let exit_code = real_main();
    std::process::exit(exit_code);
}

fn real_main() -> i32 {
    let args = get_args();

    let match_regex = args.value_of("match_regex").unwrap();
    let rename_regex = args.value_of("rename_regex").unwrap();
    let dry_run = args.is_present("dry_run");

    let re = Regex::new(match_regex).unwrap();

    let files: Vec<&str> = args.values_of("files").unwrap().collect();
    let mut new_files = Vec::with_capacity(files.len());
    let mut hs = std::collections::HashSet::with_capacity(files.len());

    for file in &files {
        new_files.push(re.replace_all(file, rename_regex));
    }

    for new_file in &new_files {
        hs.insert(new_file);
    }

    if hs.len() < new_files.len() {
        println!("Collision exists in new file names. Aborting...");
        return 1
    }

    for (file, new_file) in files.iter().zip(&new_files) {
        if file == new_file {
            continue
        }

        println!("{} => {}", file, new_file);

        if dry_run == false {
            std::fs::rename(file, new_file.to_string()).unwrap();
        }
    }

    return 0
}

fn get_args<'a>() -> ArgMatches<'a> {
    App::new("mvr")
        .version("0.1")
        .author("Collin Styles <collingstyles@gmail.com")
        .about("The mv coreutil but with regexes")
        .arg(Arg::with_name("match_regex")
                 .index(1)
                 .required(true))
        .arg(Arg::with_name("rename_regex")
                 .index(2)
                 .required(true))
        .arg(Arg::with_name("files")
                 .multiple(true)
                 .required(true))
        .arg(Arg::with_name("dry_run")
                 .short("n")
                 .long("dry-run"))
        .get_matches()
}
