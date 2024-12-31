use iced::Application;
use iced::Settings;
mod desktop_widget;
mod io_handler;
mod meta_handler;
mod note;

fn main() -> iced::Result {
    desktop_widget::Widget::run(Settings::default())
}
