extern crate gtk;

use gtk::prelude::*;
mod gui;

fn main() {
    gui::init();
    println!("Hello, world!");
    gtk::main();
}
