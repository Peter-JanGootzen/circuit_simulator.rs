use gtk::prelude::*;
use gtk::*;
use crate::models::circuit::Circuit;
use crate::models::visitor::ConcreteVisitor;
use crate::models::visitor::Visitable;

//pub fn init<T: 'static, B: 'static>(mut start_circuit_loader: B, mut retrieve_circuit: T) where T: FnMut() -> &'static Option<Circuit>, B: FnMut(String) -> Option<&'static str> {
//    if gtk::init().is_err() {
//        println!("Failed to initialize GTK.");
//        return;
//    }
//
//    let window = Window::new(WindowType::Toplevel);
//    window.set_title("Circuit Simulator");
//    window.set_default_size(500, 720);
//
//    // When the window closes, end the program.
//    window.connect_delete_event(|_, _| {
//        gtk::main_quit();
//        Inhibit(false)
//    });
//
//    let open_file_button = Button::new_with_label("Click me!");
//    let output = Label::new("Click 'Run the simulation'!");
//    let run_simulation = Button::new_with_label("Run the simulation");
//
//    window.add(&output);
//    window.add(&run_simulation);
//    window.add(&open_file_button);
//
//    run_simulation.connect_clicked(move |_| {
//        output.set_text("Simulation is loading...");
//        output.set_text(simulate(retrieve_circuit()).as_str());
//    });
//
//    open_file_button.connect_clicked(move |_| {
//        //Call function from main to load the circuit
//        match open_file_chooser() {
//            Ok(filepath) => { 
//                match start_circuit_loader(filepath) {
//                    Some(error) => show_error(error),
//                    None => ()
//                };
//            },
//            Err(error) => { return show_error(error); }
//        }
//    });
//
//    window.show_all();
//}

pub fn simulate(circuit_option: &Option<Circuit>) -> String {
    match circuit_option {
        Some(circuit) => {
            //do some visitor things and show the output.
            let mut visitor = ConcreteVisitor::new();
            let mut output = String::new();
            for node in circuit.get_nodes().iter() {
                node.borrow().accept_visitor(&mut visitor);
                output += &visitor.get_output();
            }
            return output;
        },
        None => String::from("Het circuit kon niet geladen worden")
    }
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
            Ok(filepath) => filepath,
            Err(_) => { return Err("Could not get path of file"); }
        };
    } else if res == ResponseType::Cancel.into() {
        filechooser.destroy();
    }
    filechooser.destroy();
    return Ok(filepath);
}
