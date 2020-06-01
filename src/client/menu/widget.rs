use sfml::{
    graphics::{
        Drawable,
        RenderStates,
        RenderTarget,
    },
    system::{
        Vector2i,
    }
};

use crate::menu::button::Button;
use crate::menu::label::Label;
use crate::menu::text_field::TextField;

pub enum Widget<'a, T> where T: Clone {
    Button(Button<'a, T>),
    TextField(TextField<'a, T>),
    Label(Label<'a, T>),
}

impl<'a, T> Widget<'a, T> where T: Clone {
    pub fn contains(&self, point: Vector2i) -> bool {
        match self {
            Widget::Button::<T>(button) => button.contains(point),
            Widget::TextField::<T>(text_field) => text_field.contains(point),
            Widget::Label::<T>(label) => label.contains(point),
        }
    }

    pub fn hover(&mut self, on: bool) {
        match self {
            Widget::Button::<T>(button) => button.hover(on),
            Widget::TextField::<T>(text_field) => text_field.hover(on),
            Widget::Label::<T>(label) => label.hover(on),
        }
    }

    pub fn focus(&mut self, on: bool) {
        match self {
            Widget::Button::<T>(button) => button.focus(on),
            Widget::TextField::<T>(text_field) => text_field.focus(on),
            Widget::Label::<T>(label) => label.focus(on),
        }
    }

    pub fn focused(&self) -> bool {
        match self {
            Widget::Button::<T>(button) => button.focused(),
            Widget::TextField::<T>(text_field) => text_field.focused(),
            Widget::Label::<T>(label) => label.focused(),
        }
    }

    pub fn click(&self) -> Option<T> {
        match self {
            Widget::Button::<T>(button) => button.click(),
            Widget::TextField::<T>(text_field) => text_field.click(),
            Widget::Label::<T>(label) => label.click(),
        }
    }

    pub fn set_position(&mut self, position: (f32, f32)) {
        match self {
            Widget::Button::<T>(button) => button.set_position(position),
            Widget::TextField::<T>(text_field) => text_field.set_position(position),
            Widget::Label::<T>(label) => label.set_position(position),
        }
    }

    pub fn data(&self) -> Option<&str> {
        match self {
            Widget::Button::<T>(button) => button.data(),
            Widget::TextField::<T>(text_field) => text_field.data(),
            Widget::Label::<T>(label) => label.data(),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Widget::Button::<T>(button) => button.id(),
            Widget::TextField::<T>(text_field) => text_field.id(),
            Widget::Label::<T>(label) => label.id(),
        }
    }

    pub fn enter_char(&mut self, unicode: char) {
        match self {
            Widget::Button::<T>(button) => button.enter_char(unicode),
            Widget::TextField::<T>(text_field) => text_field.enter_char(unicode),
            Widget::Label::<T>(label) => label.enter_char(unicode),
        }
    }
}

impl<'a, T> Drawable for Widget<'a, T> where T: Clone {
    fn draw<'s: 'shader, 'texture, 'shader, 'shader_texture>(
        &'s self,
        render_target: &mut dyn RenderTarget,
        states: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        match self {
            Widget::Button(button) => button.draw(render_target, states),
            Widget::TextField(text_field) => text_field.draw(render_target, states),
            Widget::Label(label) => label.draw(render_target, states),
        }
    }
}

pub trait WidgetTrait<T> where T: Clone {
    fn contains(&self, point: Vector2i) -> bool;
    fn hover(&mut self, on: bool);
    fn focus(&mut self, on: bool);
    fn click(&self) -> Option<T>;
    fn set_position(&mut self, position: (f32, f32));
    fn data(&self) -> Option<&str>;
    fn id(&self) -> &str;
    fn enter_char(&mut self, unicode: char);
    fn focused(&self) -> bool;
}
