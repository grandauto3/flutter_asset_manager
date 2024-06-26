#[allow(unused_imports)]
use iced::{
    Element,
    widget::{
        column,
        text,
        text_input,
        button,
        row,
    },
    Alignment,
    Length,
};
use iced_aw::Bootstrap;

#[allow(unused_imports)]
use crate::{
    core::{
        messages::*,
        app::AppState,
    }
};

pub struct IcedUi;

impl IcedUi {
    #[allow(unused)]
    pub fn view(state: &AppState) -> Element<AppMessages> {
        column![
            Self::create_path_input("Pubspec Path:", FILE_PATH_MESSAGE),
            Self::create_path_input("Asset Folder Path:", DIR_PATH_MESSAGE),
            button("Add assets").on_press(ADD_ASSETS_MESSAGE),
        ].align_items(Alignment::Center)
            .spacing(5)
            .into()
    }

    fn create_path_input<'a>(label: &str, message: AppMessages) -> Element<'a, AppMessages> {
        let folder_icon = iced_aw::core::icons::bootstrap::icon_to_text(Bootstrap::Foldertwo);
        row![
            text(label),
            text_input("","").width(Length::Fixed(100f32)),
            button(folder_icon).on_press(message),
        ].align_items(Alignment::Center)
            .spacing(5)
            .into()
    }
}