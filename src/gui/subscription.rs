use iced::Subscription;
use super::Gui;
use crate::message::Message;
use iced::{keyboard, event};
use iced::keyboard::key::{Code, Physical};

impl Gui {
    pub fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _window| {
            match event {
                iced::Event::Keyboard(keyboard::Event::KeyPressed {
                    key,
                    physical_key,
                    modifiers,
                    ..
                }) => {
                    // Si la touche physique est celle du pavé numérique,
                    // on ignore complètement l'interprétation "flèche".
                    let is_numpad = matches!(
                        physical_key,
                        Physical::Code(Code::Numpad0)
                            | Physical::Code(Code::Numpad1)
                            | Physical::Code(Code::Numpad2)
                            | Physical::Code(Code::Numpad3)
                            | Physical::Code(Code::Numpad4)
                            | Physical::Code(Code::Numpad5)
                            | Physical::Code(Code::Numpad6)
                            | Physical::Code(Code::Numpad7)
                            | Physical::Code(Code::Numpad8)
                            | Physical::Code(Code::Numpad9)
                    );

                    if is_numpad {
                        return None; // laisser le text_input gérer le chiffre normalement
                    }

                    match key.as_ref() {
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
                    }
                }
                _ => None,
            }
        })
    }
}