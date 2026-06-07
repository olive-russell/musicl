
mod common;

#[test]
fn remove_rejects_non_music_file() {
    common::rejects_non_music_file("remove");
}

#[test]
fn remove_moves_good_file() {
    common::moves_good_file("remove");
}

#[test]
fn remove_updates_status() {
    common::updates_status("remove");
}

#[test]
fn remove_only_appends_to_status() {
    common::only_appends_to_status("remove");
}

#[test]
fn remove_rejects_path_outside_library() {
    common::rejects_path_outside_library("remove");
}

#[test]
fn remove_rejects_path_at_root() {
    common::rejects_path_at_root("remove");
}

#[test]
fn remove_rejects_multiple_files() {
    common::rejects_multiple_files("remove");
}

#[test]
fn remove_moves_paired_lrc() {
    common::moves_paired_lrc("remove");
}