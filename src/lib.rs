use core::panic;
use std::{fs::{File, OpenOptions, canonicalize, read_dir, remove_dir}, io::Write, path::PathBuf};
use anyhow::{anyhow, Result, bail};
use chrono::Local;
use glob::glob;
use id3::{Tag, TagLike};
use serde::Deserialize;
use filenamify::filenamify;

pub fn in_library(path: &PathBuf) -> Result<bool> {
    Ok(path.starts_with(library_path()?))
}

pub fn in_archive(path: &PathBuf) -> Result<bool> {
    Ok(path.starts_with(archive_path()?))
}

pub fn library_path() -> Result<PathBuf> {
    Ok(canonicalize(PathBuf::from("./library"))?)
}

pub fn archive_path() -> Result<PathBuf> {
    Ok(canonicalize(PathBuf::from("./archive"))?)
}

pub fn status_path() -> Result<PathBuf> {
    Ok(canonicalize(PathBuf::from("./.musicl/status"))?)
}

pub fn is_music_file(path: &PathBuf) -> bool {
    valid_music_files().contains(&path.extension().unwrap().to_str().unwrap())
}

pub fn valid_music_files() -> [&'static str; 1] {
    ["mp3"]
}

pub fn isrc_in_use(path: &PathBuf) -> Result<bool> {
    let isrc = get_isrc(path).unwrap();
    let status = find_current_status(&isrc);
    if status.is_none() || status.unwrap().action == "remove" {
        Ok(false)
    } else {
        Ok(true)
    }
}

pub fn find_current_status(isrc: &String) -> Option<Status> {
    let mut status_all = get_status_all().unwrap();
    status_all.reverse();
    let status = status_all.into_iter().find(|status| status.isrc == *isrc);
    status
}

pub fn has_correct_location(path: &PathBuf) -> Result<bool> {
    Ok(*path == get_correct_location(path)?)
}

pub fn get_correct_location(path: &PathBuf) -> Result<PathBuf> {
    // Get metadata, pull out leaf details and sanitise
    let metadata = get_metadata(path).unwrap();
    let artist = filenamify(metadata.artist.unwrap());
    let album = filenamify(metadata.album.unwrap());
    let disc_string = if metadata.total_discs.is_some() && metadata.total_discs.unwrap() > 1 {format!("{} ", metadata.disc.unwrap())} else {format!("")};
    
    // Get sublibrary
    let isrc = metadata.isrc.unwrap();
    let status = find_current_status(&isrc).unwrap();
    let sublibrary = sublibrary_from_action(status.action.as_str())?;
    
    // Assemble final path
    let mut location = sublibrary.clone();
    let file_name = filenamify(format!("{}{}.{}", disc_string, metadata.title.unwrap(), path.extension().unwrap().to_str().unwrap()));
    location.extend([artist, album, file_name]);

    // Check still within working directory
    let working_dir = std::env::current_dir()?.canonicalize()?;
    if !location.starts_with(&working_dir) {
        bail!("Destination escapes working directory");
    }

    Ok(location)
}

pub fn move_music(path: &PathBuf, action: &str) -> Result<()> {
    update_status(&path, &action)?;
    move_to_correct_location(&path)?;
    Ok(())
}

pub fn sublibrary_from_action(action: &str) -> Result<PathBuf> {
    Ok(match action {
        "add" => library_path()?,
        "archive" => archive_path()?,
        "unarchive" => library_path()?,
        _ => panic!("Unimplemented sub-command"),
    })
}

// pub fn action_from_sublibrary(sublibrary: &PathBuf) -> &str {
//     match action {
//         library_path() => library_path(),
//         archive_ => archive_path(),
//         "unarchive" => library_path(),
//         _ => panic!("Unimplemented sub-command"),
//     }
// }

pub fn move_to_correct_location(path: &PathBuf) -> Result<()> {
    // bug: never created all this folders
    std::fs::rename(path, get_correct_location(path)?)?;
    Ok(())
}


pub fn remove_music(path: &PathBuf) -> Result<()> {
    remove_file(path)?;
    update_status(path, "remove")?;
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

pub fn get_status_all() -> Result<Vec<Status>> {
    // bug: where does path go? it used to be stdio by accident
    let file = File::open(status_path()?)?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let records: Vec<Status> = reader
        .deserialize()
        .collect::<Result<_, csv::Error>>()?;

    Ok(records)
}

pub fn write_status(date: String, isrc: &str, action: &str) -> Result<()> {
    OpenOptions::new().append(true).open(status_path()?)?
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
    for entry in read_dir(sublibrary)? {
        remove_empty_directories_recursive(&entry.unwrap().path())?
    }
    Ok(())
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
    if path.starts_with(library_path()?) {
        Ok(library_path()?)
    } else if path.starts_with(archive_path()?) {
        Ok(archive_path()?)
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
    let tag = Tag::read_from_path(path)?;

    Ok(Metadata {
        title: tag.title().map(String::from),
        artist: tag.artist().map(String::from),
        album: tag.album().map(String::from),
        disc: tag.disc(),
        total_discs: tag.total_discs(),
        track: tag.track(),
        isrc: tag.get("TSRC").and_then(|frame| frame.content().text()).map(str::to_owned),
        has_cover: tag.pictures().next().is_some()
    })
}

#[derive(Debug, Deserialize)]
pub struct Status {
    pub date: String,
    pub isrc: String,
    pub action: String
}

pub struct Metadata {
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    disc: Option<u32>,
    total_discs: Option<u32>,
    track: Option<u32>,
    isrc: Option<String>,
    has_cover: bool
}