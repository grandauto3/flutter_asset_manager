use iced::{
    Application,
    Command,
    Element,
    font,
    Length,
    Theme,
    widget::{
        Container
    },
};

use crate::{
    core::{
        messages::UiIcedMessage,
        messages::AppMessages,
    },
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

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = AppMessages;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        (Self::default(), Command::batch(vec![
            font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(AppMessages::FontLoaded)
        ]))
    }

    fn title(&self) -> String {
        "Window title".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Self::Message::UiMessages(UiIcedMessage::ButtonPressed(_)) => {
                self.get_state_mut().counter += 1;
                Command::none()
            }
            _ => { Command::none() }
        }
    }


    fn view(&self) -> Element<Self::Message> {
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
