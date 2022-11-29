
use walkdir::WalkDir;
use std::collections::HashMap;
use anyhow::{Result, bail};
use colored::Colorize;

//TODO: This needs to move to utils
pub fn get_files_in(dir:&str) -> Result<HashMap<String, Vec<u8>>> {
    let mut files = HashMap::new();
    for entry_result in WalkDir::new(dir) {
        let entry = match entry_result {
            Ok(x) => x,
            Err(e) => bail!("{}\n{}", "failled to get item in directory, failed with error:".red(),  e)
        };
        if entry.path().is_file() {
            let file_data = match std::fs::read(entry.path()) {
                Ok(x) => x,
                Err(e) => bail!("{}\n{}", "failed to read file in directory into a byte vector, failed with error:".red(),  e)
            };
            let path = match entry.path().to_str() {
                Some(x) => x,
                None => bail!("failed to get path as string a file")
            }.to_string();
            files.insert(path, file_data);
        }
    }
    return anyhow::Ok(files);
}
//TODO: This needs to move to utils
pub fn get_dirs_in(root:&str) -> Result<Vec<String>> {
    let mut dirs = vec![];
    for entry_result in WalkDir::new(root) {
        let entry = match entry_result {
            Ok(x) => x,
            Err(e) => bail!("{}\n{}", "failled to get item in directory, failed with error:".red(),  e)
        };
        if entry.path().is_dir() {
            let path = match entry.path().to_str() {
                Some(x) => x,
                None => bail!("failed to get path as string a file")
            }.to_string();
            dirs.push(path);
        }
    }
    return anyhow::Ok(dirs);
}
//TODO: This needs to move to utils
pub fn get_name_from_path(path:&str, include_extention:bool) -> String{
    let file_name = path
        .split("/")
        .filter(|seg|!seg.is_empty())
        .last()
        .unwrap();
    if !include_extention{
        let dot_parts = file_name.split(".").map(|s|s.to_string());
        return match dot_parts.clone().count() {
            1 => dot_parts.collect::<Vec<_>>()[0].to_owned(), //this handles the case in-which there is no file extention
            _ => {
                //This collects all but the last segment into a single string
                dot_parts.fold([String::new(), String::new()], |accum, current| {
                    [format!("{}.{}", accum[0], accum[1]), current.to_string()]
                })[0].to_owned()
            }
        }
        
    }else {
        return file_name.to_string();
    }
}