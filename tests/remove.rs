
mod common;

#[test]
fn add_rejects_non_music_file() {
    common::rejects_non_music_file("remove");
}
