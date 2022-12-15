use std::collections::HashMap;

use anyhow::{Result, bail};
use colored::Colorize;
use walkdir::WalkDir;

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