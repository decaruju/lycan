use sfml::{
    graphics::{
        Font,
        RenderWindow,
    },
};

use crate::menu::{
    menu::Menu,
    button::Button,
};

#[derive(Clone)]
pub enum MainMenuChoice {
    Quit,
    NewGame,
    JoinGame,
}

pub fn main_menu(window: &mut RenderWindow, font: &Font) -> MainMenuChoice {
    let mut menu = Menu::new();
    menu.add_button(
        Button::new(
            "New game",
            &font,
            MainMenuChoice::NewGame,
        ),
    );
    menu.add_button(
        Button::new(
            "Join game",
            &font,
            MainMenuChoice::JoinGame,
        ),
    );
    menu.add_button(
        Button::new(
            "Quit",
            &font,
            MainMenuChoice::Quit,
        ),
    );
    menu.handle(window)
}
