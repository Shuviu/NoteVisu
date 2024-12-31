use crate::note::{self, Note};
use iced::widget::Column;
use iced::widget::Text;
use iced::Application;
use iced::*;
use std::path::Path;
use std::usize;
use widget::Button;
use widget::Checkbox;
use widget::Row;

#[derive(Debug, Default)]
pub enum Mode {
    #[default]
    Viewing,
    FocusEdit,
}

#[derive(Debug, Clone)]
pub enum Message {
    SwitchToFocusEdit,
    ApplyFocusEdit,
    ToggleFocus(usize),
}

#[derive(Default)]
pub struct Widget {
    notes: Vec<note::Note>,
    current_mode: Mode,
}

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
        match message {
            Message::SwitchToFocusEdit => {
                self.current_mode = Mode::FocusEdit;
            }
            Message::ApplyFocusEdit => {
                self.current_mode = Mode::Viewing;
            }
            Message::ToggleFocus(index) => {
                self.notes[index].is_focused = !self.notes[index].is_focused;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut column = Column::new();

        column = column.push(
            Row::new().push(Button::new("Manage Focus").on_press(Message::SwitchToFocusEdit)),
        );

        match self.current_mode {
            Mode::Viewing => {
                for note in self.notes.iter() {
                    if !note.is_focused {
                        continue;
                    }
                    let text: Text = Text::new(&note.title).size(20);
                    column = column.push(text);
                }
            }
            Mode::FocusEdit => {
                for (index, note) in self.notes.iter().enumerate() {
                    let mut row = Row::new();
                    row = row
                        .push(
                            Checkbox::new("", note.is_focused)
                                .on_toggle(move |_state| Message::ToggleFocus(index)),
                        )
                        .push(Text::new(&note.title));
                    column = column.push(row);
                }

                column = column.push(Button::new("Apply").on_press(Message::ApplyFocusEdit))
            }
        }

        column.into()
    }
}
