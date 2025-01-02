use crate::desktop_widget::Widget;
use crate::meta_handler;
use crate::note::Note;
use std::fs;
use std::path::{Path, PathBuf};

pub fn read_stored_notes(path: &Path, mut application: Widget) -> Widget {
    // read in all filenames in the directory
    for entry in path.read_dir().expect("") {
        if let Ok(entry) = entry {
            let mut filename: String = entry.file_name().to_string_lossy().to_string();
            filename = filename
                .split('.')
                .next()
                .expect("No Filename Found")
                .to_string();
            let file_path: String = entry.path().to_string_lossy().to_string();

            application.notes.push(Note::new(
                filename,
                file_path,
                read_note_body(entry.path()),
                meta_handler::check_for_metatag(
                    entry.path(),
                    String::from("user.focus"),
                    String::from("true"),
                ),
                meta_handler::check_for_metatag(
                    entry.path(),
                    String::from("user.completed"),
                    String::from("true"),
                ),
            ));
        }
    }

    application
}

fn read_note_body(path: PathBuf) -> String {
    let body: Vec<u8> = fs::read(path).expect("Could not read the file");
    String::from_utf8(body).expect("Error While Converting Body to String")
}
