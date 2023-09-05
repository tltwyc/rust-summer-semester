mod walk_tree;
mod optional;
use walk_tree::walk_tree;
use optional::Optional;
use regex::Regex;
use std::env;
use std::path::Path;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args.len() > 4 {
        eprintln!("Please use myfind as: {} <path> <regex> <optional: -d/-f>", args[0]);
        eprintln!("-d: only search for directories");
        eprintln!("-f: only search for files");
        process::exit(1);
    }

    let optc: Option<&str> = args.get(3).map(|x| x.as_str());
    let opt = match optc {
        Some("-d") => Optional::D,
        Some("-f") => Optional::F,
        None => Optional::N,
        _ => {
            eprintln!("Invalid Option Code! Please use -d/-f!");
            process::exit(1);
        }
    };
    
    let pattern = &args[2];
    let regex = match Regex::new(pattern){
        Ok(re) => re,
        Err(err) => {
            eprintln!("Invalid Regex '{}': {}", pattern, err);
            process::exit(1);
        }
    };

    match find(&args[1], &regex, &opt){
        Ok(matches) => {
            if matches.is_empty(){
                println!("No matching items.");
            } else {
                println!("Items found: ");
                for file in matches {
                    println!("{}", file);
                }
            }
        }
        Err(error) => {
            eprintln!("Runtime Error: {}", error);
            process::exit(1)
        }
    }
}

fn find<P: AsRef<Path>>(root: P, regex: &Regex, opt: &Optional) -> Result<Vec<String>, Box<dyn std::error::Error>>{
    let mut matches = Vec::new();
    walk_tree(root.as_ref(), regex, opt, &mut matches)?;
    matches.sort();
    Ok(matches)
}
