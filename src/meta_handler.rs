use std::borrow::Cow;
use std::path::{Path, PathBuf};

pub fn check_for_metatag(path: PathBuf, tag: String, expected_tag_value: String) -> bool {
    match xattr::get(path, tag) {
        Ok(Some(value)) => {
            let tag_value: Cow<'_, str> = String::from_utf8_lossy(&value);

            if tag_value.to_string() == expected_tag_value {
                println!("Found: {}", tag_value);
                return true;
            } else {
                print!("Found: {}", tag_value);
                return false;
            }
        }
        Ok(None) => {
            println!("Tag does not have a value");
            return false;
        }
        Err(_) => {
            println!("Could not read tag");
            return false;
        }
    }
}

pub fn set_metatag(path: &Path, tag: String, new_tag_value: String) -> bool {
    match xattr::set(path, tag, new_tag_value.as_bytes()) {
        Ok(_) => return true,
        Err(_) => return false,
    };
}
