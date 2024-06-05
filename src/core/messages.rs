use iced::font;

pub const FILE_PATH_MESSAGE: AppMessages = AppMessages::UiMessages(
    UiIcedMessage::ButtonPressed(
        Some(ButtonPressedMessage::FilePath)));
pub const DIR_PATH_MESSAGE: AppMessages = AppMessages::UiMessages(
    UiIcedMessage::ButtonPressed(
        Some(ButtonPressedMessage::DirPath)));

#[derive(Debug, Clone)]
pub enum AppMessages {
    FontLoaded(Result<(), font::Error>),
    UiMessages(UiIcedMessage)
}

#[derive(Debug, Clone)]
pub enum UiIcedMessage {
    ButtonPressed(Option<ButtonPressedMessage>),
}

#[derive(Debug, Clone)]
pub enum ButtonPressedMessage {
    FilePath,
    DirPath,
}