use cursive::{
    event::{Event, Key},
    theme::{BaseColor, Color},
};

#[derive(Clone)]
pub struct Keybindings {
    pub down: Event,
    pub up: Event,
    pub left: Event,
    pub right: Event,

    pub focus_next: Event,
    pub focus_prev: Event,
}

impl Keybindings {
    pub fn new() -> Self {
        Self {
                down: Event::Key(Key::Down),
                up: Event::Key(Key::Up),
                left: Event::Key(Key::Left),
                right: Event::Key(Key::Right),

                focus_next: Event::Key(Key::Tab),
                focus_prev: Event::Shift(Key::Tab),
        }
    }
}

#[derive(Copy, Clone)]
pub enum BorderStyle {
    Default,
    Light,
    Heavy,
    Round,
}
