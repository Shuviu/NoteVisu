#[derive(Debug, Clone)]
pub struct Note {
    pub title: String,
    pub path: String,
    pub is_completed: bool,
    pub is_focused: bool,
}

impl Note {
    pub fn new(title: String, path: String, is_focused: bool, is_completed: bool) -> Self {
        Note {
            title,
            path,
            is_completed,
            is_focused,
        }
    }
}
