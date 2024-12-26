use iced::widget::column;
use iced::widget::text;
use iced::widget::Column;

use crate::note;

#[derive(Default)]
pub struct Widget {
    notes: Vec<note::Note>,
}

impl Widget {
    pub fn update(&mut self, message: Message) {}

    pub fn view(&self) -> Column<Message> {
        let note = text("test");

        let interface = column![note];

        interface
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Reload,
}
