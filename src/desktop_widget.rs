use crate::io_handler;
use crate::meta_handler;
use crate::note::{self, Note};
use iced::widget::Column;
use iced::widget::Text;
use iced::Application;
use iced::*;
use std::path::Path;
use std::path::PathBuf;
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
    ViewNote(Note),
    ViewCompleted,
}

// Messages
#[derive(Debug, Clone)]
pub enum Message {
    SwitchToFocusEdit,
    SwitchToViewing,
    ToggleFocus(usize),
    ViewNote(usize),
    EditNote(usize),
    ViewCompleted,
    ToggleCompleted(usize),
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
    type Theme = iced_style::Theme;

    fn theme(&self) -> Self::Theme {
        iced_style::Theme::TokyoNight
    }

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
            Message::SwitchToViewing => {
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
            Message::ViewNote(index) => {
                self.current_mode = Mode::ViewNote(self.notes[index].clone())
            }
            Message::EditNote(index) => {
                let mut nvim_process = std::process::Command::new("cosmic-term")
                    .arg("--")
                    .arg("nvim")
                    .arg(&self.notes[index].path)
                    .spawn()
                    .expect("could not start nvim");

                match nvim_process.wait() {
                    Ok(status) => {
                        if status.success() {
                            println!("Exited nvim_process successfully");
                        } else {
                            println!("Errow while exiting nvim_process")
                        }
                    }
                    Err(e) => println!("{}", e),
                }
            }
            Message::ViewCompleted => self.current_mode = Mode::ViewCompleted,
            Message::ToggleCompleted(index) => {
                self.notes[index].is_completed = !self.notes[index].is_completed;
                meta_handler::set_metatag(
                    Path::new(self.notes[index].path.as_str()),
                    String::from("user.completed"),
                    self.notes[index].is_completed.to_string(),
                );
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut main_pane = Row::new();
        let mut sidebar = Column::new().padding(20);
        let mut view_screen = Column::new().padding(20);
        // create the universal header layout
        sidebar = sidebar
            .push(Button::new("Show Focused").on_press(Message::SwitchToViewing))
            .push(Button::new("Manage Focus").on_press(Message::SwitchToFocusEdit))
            .push(Button::new("Show Completed").on_press(Message::ViewCompleted))
            .push(Button::new("Analytics"));

        match self.current_mode {
            // Create the viewing mode layout
            Mode::Viewing => {
                view_screen = view_screen.push(Text::new("{ \n  FocusedToDos: [ \n"));
                for (index, note) in self.notes.iter().enumerate() {
                    if !note.is_focused {
                        continue;
                    }
                    let mut row = Row::new().padding(10);
                    let note_title: Text = Text::new(&note.title);
                    let view_btn = Button::new("View Content")
                        .on_press(Message::ViewNote(index))
                        .padding(10);
                    let edit_btn = Button::new("Edit Note")
                        .on_press(Message::EditNote(index))
                        .padding(10);
                    let done_btn = Button::new("Done")
                        .on_press(Message::ToggleCompleted(index))
                        .padding(10);
                    row = row
                        .push(Text::new("            { ToDo: "))
                        .push(note_title)
                        .push(view_btn)
                        .push(edit_btn)
                        .push(done_btn)
                        .push(Text::new("}, "));

                    view_screen = view_screen.push(row);
                }

                view_screen = view_screen.push(Text::new("         ]"));
                view_screen = view_screen.push(Text::new("}"));
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
                    view_screen = view_screen.push(row);
                }
                // Add footer row
                view_screen =
                    view_screen.push(Button::new("Apply").on_press(Message::SwitchToViewing));
            }
            Mode::ViewNote(ref note) => {
                view_screen = view_screen.push(Text::new(io_handler::read_note_body(
                    PathBuf::from(&note.path),
                )));
                view_screen =
                    view_screen.push(Button::new("Done").on_press(Message::SwitchToViewing))
            }

            Mode::ViewCompleted => {
                view_screen = view_screen.push(Text::new("{ \n  FocusedToDos: [ \n"));
                for (index, note) in self.notes.iter().enumerate() {
                    if !note.is_completed {
                        continue;
                    }
                    let mut row = Row::new();
                    let note_title: Text = Text::new(&note.title);
                    let view_btn = Button::new("View Content").on_press(Message::ViewNote(index));
                    let edit_btn = Button::new("Edit Note").on_press(Message::EditNote(index));
                    row = row
                        .push(Text::new("            { ToDo: "))
                        .push(note_title)
                        .padding(5)
                        .push(view_btn)
                        .padding(5)
                        .push(edit_btn)
                        .padding(5)
                        .push(Text::new("}, "));

                    view_screen = view_screen.push(row);
                }

                view_screen = view_screen.push(Text::new("         ]"));
                view_screen = view_screen.push(Text::new("}"));
            }
        }
        main_pane = main_pane.push(sidebar).push(view_screen);
        main_pane.into()
    }
}
