use crate::note::{self, Note};
use iced::widget::Column;
use iced::Application;
use iced::*;
use std::array::from_fn;
use std::fs;
use std::path::Path;
use widget::Text;

#[derive(Default)]
pub struct Widget {
    notes: Vec<note::Note>,
}

#[derive(Debug)]
pub enum Message {}

impl Widget {
    pub fn new() -> Self {
        let mut widget: Widget = Widget::default();
        let str_path: &str = "/home/shuviu/01_Data/90_Notes/";
        let path: &Path = Path::new(&str_path);

        if !path.is_dir() {
            widget
                .notes
                .push(Note::new(String::from("ALARM"), String::from("ALARM")));
            return widget;
        }

        for entry in path.read_dir().expect("") {
            if let Ok(entry) = entry {
                let filename: String = entry.file_name().to_string_lossy().to_string();
                let file_path: String = entry.path().to_string_lossy().to_string();

                widget.notes.push(Note::new(filename, file_path));
            }
        }
        widget
    }
}

impl Application for Widget {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Flags = ();
    type Theme = theme::Theme;

    fn title(&self) -> String {
        String::from("Hello World")
    }

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::new(), Command::none())
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let text: Text = Text::new(&self.notes[0].title).size(40);
        Column::new().push(text).into()
    }
}
