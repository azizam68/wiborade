mod excel;
mod files;
mod gui;
mod image_ops;
mod message;

use gui::Gui;
use iced::Task;

fn main() -> iced::Result {
    iced::application(|| (Gui::default(), Task::none()), Gui::update, Gui::view)
        .title("Gui")
        .subscription(Gui::subscription)
        .run()
}
