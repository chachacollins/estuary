use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
mod common;
use crate::common::BoW;
use crate::common::tokenize;

const HAM_TRAINING_DATA_DIRECTORY: &'static str = "dataset/training/ham";
const SPAM_TRAINING_DATA_DIRECTORY: &'static str = "dataset/training/spam";

fn read_file_and_fill_bows(entry: &DirEntry, bow: &mut BoW) -> io::Result<()> {
    let file = fs::read_to_string(entry.path())?;
    file.split_whitespace()
        .map(|x| tokenize(x))
        .filter(|x| !x.is_empty())
        .for_each(|key| {
            if let Some(val) = bow.get(&key) {
                bow.insert(key, val + 1);
            } else {
                bow.insert(key, 1);
            }
        });
    Ok(())
}

fn visit_dir_and_fill_bows(dir: &Path, bow: &mut BoW) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dir_and_fill_bows(&path, bow)?;
            } else {
                read_file_and_fill_bows(&entry, bow)?;
            }
        }
    }
    Ok(())
}

fn write_json_to_file(json: String, file_path: &str) -> io::Result<()> {
    fs::write(file_path, json)?;
    Ok(())
}

fn main() {
    let mut spam_bows = BoW::new();
    let mut ham_bows = BoW::new();
    visit_dir_and_fill_bows(Path::new(SPAM_TRAINING_DATA_DIRECTORY), &mut spam_bows).unwrap();
    visit_dir_and_fill_bows(Path::new(HAM_TRAINING_DATA_DIRECTORY), &mut ham_bows).unwrap();
    let spam_json = serde_json::to_string(&spam_bows).unwrap();
    let ham_json = serde_json::to_string(&ham_bows).unwrap();
    write_json_to_file(spam_json, "spam.json").unwrap();
    write_json_to_file(ham_json, "ham.json").unwrap();
}
