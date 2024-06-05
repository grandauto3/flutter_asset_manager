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
use iced::{Alignment, Length};
use iced_aw::Bootstrap
;
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

        ].align_items(Alignment::Center)
            .spacing(5)
            .into()
    }

    fn create_path_input<'a>() -> Element<'a, UiIcedMessage> {
        let folder_icon = iced_aw::core::icons::bootstrap::icon_to_text(Bootstrap::Folder);
        row![
            text("assetPath"),
            text_input("","").width(Length::Fixed(100f32)),
            button(folder_icon).on_press(UiIcedMessage::ButtonPressed(Some(ButtonPressedMessage::FilePath))),
        ].align_items(Alignment::Center)
            .spacing(5)
            .into()
    }
}