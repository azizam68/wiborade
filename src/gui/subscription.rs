use iced::Subscription;
use super::Gui;
use crate::message::Message;
use iced::{keyboard, event};

impl Gui {
pub fn subscription(&self) -> Subscription<Message> {
    event::listen_with(|event, _status, _window| {
        match event {
            iced::Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                modifiers,
                ..
            }) => match key.as_ref() {
                keyboard::Key::Named(keyboard::key::Named::Tab) => {
                    if modifiers.shift() {
                        Some(Message::FocusPrevious)
                    } else {
                        Some(Message::FocusNext)
                    }
                }
                keyboard::Key::Named(keyboard::key::Named::ArrowDown) => {
                    Some(Message::FocusNext)
                }
                keyboard::Key::Named(keyboard::key::Named::ArrowUp) => {
                    Some(Message::FocusPrevious)
                }
                _ => None,
            },
            _ => None,
        }
    })
}
}