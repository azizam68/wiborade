mod gui;
use gui::Gui;
use iced::Task;
fn main() -> iced::Result {
    iced::application(
        || (Gui::default(), Task::none()), Gui::update, Gui::view)
        .title("Gui")
        .run()
}