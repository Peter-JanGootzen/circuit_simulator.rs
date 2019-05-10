use gtk::prelude::*;
use gtk::{Button, Window, WindowType, FileChooserDialog, FileChooserAction, ResponseType};
use std::process;
use std::fs;

pub fn init(){
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Circuit Simulator");
    window.set_default_size(500, 720);

    // When the window closes, end the program.
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let button = Button::new_with_label("Click me!");
    button.connect_clicked(|_| {
        open_file_chooser()
    });

    window.add(&button);

    window.show_all();
}

pub fn open_file_chooser() {
    let filechooser = FileChooserDialog::new("Open File",
                            Some(&Window::new(WindowType::Popup)),
                            FileChooserAction::Open,
                   );
    filechooser.add_button("Cancel", ResponseType::Cancel.into());
    filechooser.add_button("Open", ResponseType::Ok.into());
    let res = filechooser.run();
    if res == ResponseType::Ok.into() {
        let filename = filechooser.get_filename();
        let file = fs::read_to_string(filename.unwrap().into_os_string());
        println!("{:?}", file);
    } else if res == ResponseType::Cancel.into() {
        println!("You canceled the file dialog");
    }
    filechooser.destroy();
    process::exit(0);
}
