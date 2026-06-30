use iced::widget::{column};
use iced::{Element};

#[derive(Default)]
pub struct Gui {

}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    
}

impl Gui {
    pub fn view(&self) -> Element<Message> {
        column![].into()
    }
    
    pub fn update(&mut self, message: Message) {

    }
}
