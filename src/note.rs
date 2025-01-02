use chrono::{Local, NaiveDate};

#[derive(Debug, Clone)]
pub struct Note {
    pub title: String,
    pub path: String,
    pub creation_date: NaiveDate,
    pub is_completed: bool,
    pub is_focused: bool,
    pub body: String,
}

impl Note {
    pub fn new(
        title: String,
        path: String,
        body: String,
        is_focused: bool,
        is_completed: bool,
    ) -> Self {
        Note {
            title,
            path,
            creation_date: Local::now().date_naive(),
            is_completed,
            is_focused,
            body,
        }
    }
}
