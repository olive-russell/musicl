use std::path::PathBuf;
use anyhow::Result;

pub fn get_metadata() {

}

pub fn has_sufficient_metadata() {

}

pub fn in_library(path: &PathBuf) -> bool {
    true
}

pub fn in_archive(path: &PathBuf) -> bool {
    true
}

pub fn is_music_file(path: &PathBuf) -> bool {
    true
}

pub fn isrc_in_use(path: &PathBuf) -> bool {
    true
}

pub fn missing_metadata(path: &PathBuf) -> Vec<String> {
    vec![]
}

pub fn has_correct_location(path: &PathBuf) -> bool {
    *path == get_correct_location(path)
}

pub fn get_correct_location(path: &PathBuf) -> PathBuf {
    PathBuf::new()
}

pub fn move_music(path: &PathBuf, status: &str) -> Result<()> {
    let sublibrary = match status {
        "add" => "library",
        "archive" => "archive",
        "unarchive" => "library",
        _ => panic!("Unimplemented sub-command"),
    };

    move_file(&path, &sublibrary)?;
    update_status(&path, &status)?;
    Ok(())
}

pub fn remove_music(path: &PathBuf) -> Result<()> {
    remove_file(path)?;
    update_status(path, "remove")?;
    Ok(())
}

pub fn move_file(path: &PathBuf, sublibrary: &str) -> Result<()> {
    Ok(())
}

pub fn remove_file(path: &PathBuf) -> Result<()> {
    Ok(())
}

pub fn update_status(path: &PathBuf, status: &str) -> Result<()> {
    Ok(())
}

pub fn write_status(isrc: &str, status: &str) -> Result<()> {
    Ok(())
}

pub fn get_current_status_all() -> Vec<()> {
    get_status_all()
 }

pub fn get_status_all() -> Vec<()> {
    vec![]
}

pub fn get_music_files() -> Vec<PathBuf> {
    vec![]
}

pub fn get_lrc_files() -> Vec<PathBuf> {
    vec![]
}

pub fn get_isrc(path: &PathBuf) -> String {
    "".to_string()
}

pub fn find_current_status(current_status_all: Vec<()>, isrc: String) -> String {
    current_status_all.iter().find(|&x| x.isrc == isrc).unwrap();
    "".to_string()
}

pub fn remove_empty_directories(sublibrary: &str) {

}

pub fn get_sublibrary(path: &PathBuf) -> String {
    "".to_string()
}