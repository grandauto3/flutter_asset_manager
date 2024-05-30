use iced::{
    Element,
    widget::{
        column,
        text,
        button,
    },
};
use crate::core::app::AppState;
use crate::core::messages::UiIcedMessage;

pub struct IcedUi;

impl IcedUi {
    pub fn view(state: &AppState) -> Element<UiIcedMessage> {
        column![
            text(state.counter),
            button("Increase").on_press(UiIcedMessage::ButtonPressed),
        ].into()
    }
}