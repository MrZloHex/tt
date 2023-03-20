extern crate cursive;

use cursive::views::{Dialog, TextView};
use cursive::event::Key;
use cursive::{Cursive, CursiveExt};

mod serial;
mod ui;
mod root;
mod config;
mod panel;

use serial::Serial;
use ui::UI;


fn main() {
    println!("{:?}", serial::scan());

    //let mut ser = Serial::new().baudrate(115200).port("/dev/ttyACM0".to_string());
    //ser.open();
    //ser.write(b"$AT 00 00 0 0208^0B");
    let mut siv = Cursive::new();
    siv.add_global_callback('q', Cursive::quit);
    siv.add_global_callback(Key::Esc, Cursive::quit);
    let mut ui = UI::new(siv, config::Keybindings::new());

    ui.home();

    ui.run();

}

    //port.open();
    // $AT 00 00 0 0208^0B
    //port.write(b"$AT 00 00 0 0208^0B\r");
