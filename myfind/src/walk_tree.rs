use crate::optional::Optional;
use std::fs;
use regex::Regex;
use std::path::Path;

pub fn walk_tree(
    dir: &Path,
    regex: &Regex,
    opt: &Optional,
    matches:&mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>>{
    if dir.is_dir(){
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir(){
                walk_tree(&path, regex, opt, matches)?;
            }
            if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                if regex.is_match(filename){
                    let flag = match opt {
                        Optional::D => path.is_dir(),
                        Optional::F => path.is_file(),
                        Optional::N => true,
                    };
                    if flag {
                        matches.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    Ok(())
}