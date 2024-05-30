use iced::{
    Element,
    Length,
    Sandbox,
    Theme,
    widget::{
        Container
    },
};

use crate::{
    core::messages::UiIcedMessage,
    ui::iced_ui::IcedUi,
};

#[derive(Default)]
pub struct AppState {
    pub counter: u32,
}

#[derive(Default)]
pub struct App(AppState);

impl App {
    fn get_state(&self) -> &AppState {
        &self.0
    }

    fn get_state_mut(&mut self) -> &mut AppState {
        &mut self.0
    }
}

impl Sandbox for App {
    type Message = UiIcedMessage;
    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "Window title".to_owned()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            UiIcedMessage::ButtonPressed(_) => self.get_state_mut().counter += 1,
        }
    }


    fn view(&self) -> Element<'_, Self::Message> {
        Container::new(
            IcedUi::view(self.get_state()))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::CatppuccinMocha
    }
}
