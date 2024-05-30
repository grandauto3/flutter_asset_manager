#[allow(unused_imports)]
use iced::{
    Element,
    widget::{
        column,
        text,
        text_input,
        button,
        row,
        Row,
    },
};
use crate::{
    core::app::AppState,
    core::messages::{
        UiIcedMessage,
        ButtonPressedMessage,
    },
};

pub struct IcedUi;

impl IcedUi {
    #[allow(unused)]
    pub fn view(state: &AppState) -> Element<UiIcedMessage> {
        column![
            text(state.counter),
            button("Increase").on_press(UiIcedMessage::ButtonPressed(None)),
            Self::create_path_input()
        ].into()
    }

    fn create_path_input<'a>() -> Element<'a, UiIcedMessage> {
        row![
            text("assetPath"),
            text_input("",""),
            button("Increase").on_press(UiIcedMessage::ButtonPressed(Some(ButtonPressedMessage::FilePath))),
        ].into()
    }
}