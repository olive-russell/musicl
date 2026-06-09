use std::path::PathBuf;

pub fn get_metadata() {

}

pub fn has_sufficient_metadata() {

}

pub fn in_library(path: PathBuf) -> bool {
    true
}

pub fn in_archive(path: PathBuf) -> bool {
    true
}

pub fn is_music_file(path: PathBuf) -> bool {
    true
}

pub fn isrc_in_use(path: PathBuf) -> bool {
    true
}

pub fn missing_metadata(path: PathBuf) -> Vec<String> {
    vec![]
}

pub fn get_correct_location() {

}

pub fn move_music(path: PathBuf) -> Result<()> {
    move_file(path)?;
    update_status(path)?;
    Ok(())
}

pub fn move_file(path: PathBuf) -> Result<()> {
    Ok(())
}

pub fn update_status(path: PathBuf) -> Result<()> {
    Ok(())
}