use crate::io_handler;
use crate::meta_handler;
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

// Display Modes
#[derive(Debug, Default)]
pub enum Mode {
    #[default]
    Viewing,
    FocusEdit,
}

// Messages
#[derive(Debug, Clone)]
pub enum Message {
    SwitchToFocusEdit,
    ApplyFocusEdit,
    ToggleFocus(usize),
}

// Application Struct Containing States
#[derive(Default)]
pub struct Widget {
    pub notes: Vec<note::Note>,
    current_mode: Mode,
}

impl Widget {
    // Init function to read in locally saved notes
    pub fn new() -> Self {
        let mut widget: Widget = Widget::default();
        let str_path: &str = "/home/shuviu/01_Data/90_Notes/";
        let path: &Path = Path::new(&str_path);

        // check if path is valid
        if !path.is_dir() {
            widget.notes.push(Note::new(
                String::from("ALARM"),
                String::from("ALARM"),
                true,
                false,
            ));
            return widget;
        }

        widget = io_handler::read_stored_notes(path, widget);

        // return widget with updated state
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

    // Init function
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

                match meta_handler::set_metatag(
                    Path::new(self.notes[index].path.as_str()),
                    String::from("user.focus"),
                    self.notes[index].is_focused.to_string(),
                ) {
                    true => println!("Updated Metatag"),
                    false => println!("Error while updating Metatag"),
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut column = Column::new();

        // create the universal header layout
        column = column.push(
            Row::new().push(Button::new("Manage Focus").on_press(Message::SwitchToFocusEdit)),
        );

        match self.current_mode {
            // Create the viewing mode layout
            Mode::Viewing => {
                for note in self.notes.iter() {
                    if !note.is_focused {
                        continue;
                    }
                    let text: Text = Text::new(&note.title).size(20);
                    column = column.push(text);
                }
            }
            // Create the FocusEdit Mode layout
            Mode::FocusEdit => {
                // iterate over all notes
                for (index, note) in self.notes.iter().enumerate() {
                    let mut row = Row::new();
                    // create a new row for each note
                    row = row
                        .push(
                            // match new checkbox to the note index (State vector index)
                            Checkbox::new("", note.is_focused)
                                .on_toggle(move |_state| Message::ToggleFocus(index)),
                        )
                        // Add note title
                        .push(Text::new(&note.title));
                    column = column.push(row);
                }
                // Add footer row
                column = column.push(Button::new("Apply").on_press(Message::ApplyFocusEdit))
            }
        }

        column.into()
    }
}
