use core::panic;
use std::{fs::{OpenOptions, read_dir, remove_dir}, io::Write, path::PathBuf};
use anyhow::{anyhow, Result, bail};
use chrono::Local;
use glob::glob;
use id3::{Tag, TagLike};
use serde::Deserialize;

pub fn in_library(path: &PathBuf) -> bool {
    path.starts_with(library_path())
}

pub fn in_archive(path: &PathBuf) -> bool {
    path.starts_with(archive_path())
}

pub fn library_path() -> PathBuf {
    PathBuf::from("./library")
}

pub fn archive_path() -> PathBuf {
    PathBuf::from("./archive")
}

pub fn status_path() -> PathBuf {
    PathBuf::from("./.musicl/status")
}

pub fn is_music_file(path: &PathBuf) -> bool {
    valid_music_files().contains(&path.extension().unwrap().to_str().unwrap())
}

pub fn valid_music_files() -> [&'static str; 1] {
    ["mp3"]
}

pub fn isrc_in_use(path: &PathBuf) -> Result<bool> {
    // dev tag: would like current status
    // let isrc = get_metadata(path)?.isrc.unwrap();
    // let current_status_all = get_current_status_all()?;
    // Ok(current_status_all.iter().find(|status| status.isrc == isrc).is_some())
    let isrc = get_isrc(path).unwrap();
    // let mut status_all = get_status_all()?;
    // status_all.reverse();
    let status = find_current_status(isrc);
    if status.is_none() || status.unwrap().action == "remove" {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn find_current_status(isrc: String) -> Option<Status> {
    // current_status_all.iter().find(|&status| status.isrc == isrc).unwrap().isrc.clone()
    let mut status_all = get_status_all().unwrap();
    status_all.reverse();
    let status = status_all.into_iter().find(|status| status.isrc == isrc);
    status
}

pub fn has_correct_location(path: &PathBuf) -> Result<bool> {
    Ok(*path == get_correct_location(path, &get_sublibrary(path)?)?)
}

pub fn get_correct_location(path: &PathBuf, sublibrary: &PathBuf) -> Result<PathBuf> {
    let metadata = get_metadata(path).unwrap();
    let mut new_path = sublibrary.clone();
    new_path.extend([metadata.artist.unwrap(), metadata.album.unwrap(), format!("{}.{}",metadata.title.unwrap(),path.extension().unwrap().to_str().unwrap())]);
    Ok(new_path)
    // PathBuf::from(sublibrary).push(format!("{}/{}/{}/{}.{}", sublibrary, metadata.artist.unwrap(), metadata.album.unwrap(), metadata.title.unwrap(), path.extension().unwrap().to_str().unwrap())))
}

pub fn move_music(path: &PathBuf, action: &str) -> Result<()> {


    move_file(&path, &sublibrary_from_action(action))?;
    update_status(&path, &action)?;
    Ok(())
}

pub fn sublibrary_from_action(action: &str) -> PathBuf {
    match action {
        "add" => library_path(),
        "archive" => archive_path(),
        "unarchive" => library_path(),
        _ => panic!("Unimplemented sub-command"),
    }
}

pub fn remove_music(path: &PathBuf) -> Result<()> {
    remove_file(path)?;
    update_status(path, "remove")?;
    Ok(())
}

pub fn move_file(path: &PathBuf, sublibrary: &PathBuf) -> Result<()> {
    std::fs::rename(path, get_correct_location(path, sublibrary)?)?;
    Ok(())
}

pub fn remove_file(path: &PathBuf) -> Result<()> {
    std::fs::remove_file(path)?;
    Ok(())
}

pub fn update_status(path: &PathBuf, action: &str) -> Result<()> {
    let isrc = get_isrc(path).unwrap();
    let date = Local::now().format("%Y-%m-%d").to_string();
    write_status(date, &isrc, action)?;
    Ok(())
}

pub fn get_current_status_all() -> Result<Vec<Status>> {
    // todo!();
    let status_all = get_status_all()?;
    Ok(status_all.into_iter().filter(|status| status.action == "add").collect())
}

pub fn get_status_all() -> Result<Vec<Status>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(std::io::stdin());

    let records: Vec<Status> = reader
        .deserialize()
        .collect::<Result<_, csv::Error>>()?;

    Ok(records)
}

pub fn write_status(date: String, isrc: &str, action: &str) -> Result<()> {
    OpenOptions::new().append(true).open(status_path())?
        .write_all(format!("{},{},{}\n", date, isrc, action).as_bytes())?;
    Ok(())
}

pub fn get_music_files() -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for extension in valid_music_files() {
        files.extend(get_files_with_extension(extension)?);
    }

    Ok(files)
}

pub fn get_lrc_files() -> Result<Vec<PathBuf>> {
    get_files_with_extension("lrc")
}

pub fn get_files_with_extension(extension: &str) -> Result<Vec<PathBuf>>{
    Ok(
        glob(&format!("**/library/*.{}", extension))?
        .chain(glob(&format!("**/archive/*.{}", extension))?)
        .collect::<Result<Vec<_>, _>>()?
    )
}

pub fn get_isrc(path: &PathBuf) -> Option<String> {
    get_metadata(path).unwrap().isrc
}

pub fn remove_empty_directories(sublibrary: &PathBuf) -> Result<()> {
    remove_empty_directories_recursive(sublibrary)
}

pub fn remove_empty_directories_recursive(path: &PathBuf) -> Result<()> {
    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            remove_empty_directories_recursive(&path)?;
        }
    }
    if read_dir(path)?.next().is_none() {
        remove_dir(path)?;
    } 
    Ok(())
}

pub fn get_sublibrary(path: &PathBuf) -> Result<PathBuf> {
    if path.starts_with(library_path()) {
        Ok(library_path())
    } else if path.starts_with(archive_path()) {
        Ok(archive_path())
    } else {
        Err(anyhow!("Path not in sublibrary."))
    }
}

pub fn missing_metadata(path: &PathBuf) -> Vec<String> {
    let mut missing = vec![];

    let metadata = get_metadata(path).unwrap();

    if metadata.title.is_none() {
        missing.push(String::from("title"));
    }

    if metadata.artist.is_none() {
        missing.push(String::from("artist"));
    }

    if metadata.album.is_none() {
        missing.push(String::from("album"));
    }

    if metadata.track.is_none() {
        missing.push(String::from("track"));
    }

    if metadata.isrc.is_none() {
        missing.push(String::from("isrc"));
    }

    if !metadata.has_cover {
        missing.push(String::from("cover"));
    }

    missing
}

pub fn get_metadata(path: &PathBuf) -> Result<Metadata> {
    match path.extension().unwrap().to_str().unwrap() {
        "mp3" => get_metadata_id3(path),
        _ => bail!("File type not implemented")
    }
}

pub fn get_metadata_id3(path: &PathBuf) -> Result<Metadata> {
    let tag = Tag::read_from_path("testdata/id3v24.id3")?;

    Ok(Metadata {
        title: tag.title().map(String::from),
        artist: tag.artist().map(String::from),
        album: tag.album().map(String::from),
        disc: tag.disc(),
        track: tag.track(),
        isrc: tag.get("TSRC").and_then(|frame| frame.content().text()).map(str::to_owned),
        has_cover: tag.pictures().next().is_some()
    })
}

#[derive(Debug, Deserialize)]
pub struct Status {
    date: String,
    pub isrc: String,
    pub action: String
}

pub struct Metadata {
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    disc: Option<u32>,
    track: Option<u32>,
    isrc: Option<String>,
    has_cover: bool
}