use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use crate::models::LanguageMetadata;

pub fn get_metadata(dir: &PathBuf) -> Option<LanguageMetadata> {
    if !dir.is_dir() {
        return None;
    }

    let metadata_file = Path::new(dir).join("metadata.json");
    if !metadata_file.exists() {
        return None;
    }

    let contents = fs::read_to_string(metadata_file).unwrap();

    Some(serde_json::from_str::<LanguageMetadata>(contents.as_str()).unwrap())
}

pub fn find_metadata(lang: String) -> Option<LanguageMetadata> {
    for path in std::fs::read_dir("./packages/").unwrap() {
        let dir = path.unwrap().path();
        if !dir.is_dir() {
            continue;
        }

        if dir.ends_with(lang.clone()) {
            return get_metadata(&dir);
        }
    }

    None
}
