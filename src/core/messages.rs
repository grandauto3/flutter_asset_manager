use iced::font;

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