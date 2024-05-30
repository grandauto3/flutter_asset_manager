#[derive(Debug, Clone)]
pub enum UiIcedMessage {
    ButtonPressed(Option<ButtonPressedMessage>),
}

#[derive(Debug, Clone)]
pub enum ButtonPressedMessage{
    FilePath,
    DirPath
}