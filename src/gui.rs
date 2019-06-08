use gtk::prelude::*;
use gtk::{Button, Window, WindowType, FileChooserDialog, FileChooserAction, ResponseType};
use gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog};

pub fn init(start_circuit_loader: fn(String) -> ()){
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
    button.connect_clicked(move |_| {
        //Call function from main to load the circuit
        match open_file_chooser() {
            Ok(filepath) => { start_circuit_loader(filepath); }
            Err(error) => { return show_error(error); }
        }
    });

    window.add(&button);

    window.show_all();
}

pub fn show_error(error: &str) {
    let dialog = MessageDialog::new(None::<&Window>,
                                    DialogFlags::empty(),
                                    MessageType::Error,
                                    ButtonsType::Ok,
                                    error);
    dialog.run();
    dialog.destroy();
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
