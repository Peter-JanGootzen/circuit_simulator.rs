use gtk::prelude::*;
use gtk::{Button, Window, WindowType, FileChooserDialog, FileChooserAction, ResponseType};
use crate::circuit_file_reader;

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
        let file = println!("{:?}", circuit_file_reader::load_file(open_file_chooser().unwrap()));
    });

    window.add(&button);

    window.show_all();
}

pub fn open_file_chooser() -> Result<String, &'static str> {
    let filechooser = FileChooserDialog::new("Open File",
                            Some(&Window::new(WindowType::Popup)),
                            FileChooserAction::Open,
                   );
    filechooser.add_button("Cancel", ResponseType::Cancel.into());
    filechooser.add_button("Open", ResponseType::Ok.into());
    let res = filechooser.run();
    let mut filepath = String::new();
    if res == ResponseType::Ok.into() {
        filepath = match filechooser.get_filename().unwrap().into_os_string().into_string() {
            Ok(filepath) => { filepath }
            Err(_) => { return Err("Could not get path of file") }
        };
    } else if res == ResponseType::Cancel.into() {
        filechooser.destroy();
        return Err("You canceled the file dialog");
    }
    filechooser.destroy();
    return Ok(filepath);
}
