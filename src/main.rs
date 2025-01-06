use iced::Settings;
mod desktop_widget;
mod io_handler;
mod meta_handler;
mod note;

fn main() -> iced::Result {
    let custom_settings = Settings {
        ..Settings::default()
    };
    <desktop_widget::Widget as iced::Application>::run(custom_settings)
}
