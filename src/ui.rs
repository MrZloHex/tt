use cursive::{Cursive, CursiveExt};
use cursive::view::Resizable;

use crate::root::RootLayout;
use crate::config::Keybindings;
use crate::panel::WithPanel;

pub struct UI {
    siv: Cursive,
    keys: Keybindings
}

impl UI {
    pub fn new(siv: Cursive, keys: Keybindings) -> Self {
        Self {
            siv,
            keys
        }
    }

    pub fn run(&mut self) {
        self.siv.run()
    }

    pub fn home(&mut self) {
        self.siv.add_fullscreen_layer(
            RootLayout::vertical(self.keys.clone())
                // .child(logo_view)
                // .child(spacer_view)
                // .child(search_promt)
                .input(true)
                .full_screen()
                .with_panel()
                .title("malsa"),
        ) 
    }
}
