extern crate gtk;

use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("interface.glade");
    let builder = gtk::Builder::new_from_string(glade_src);
    let window: gtk::Window = builder.get_object("main_window").unwrap();
    let button: gtk::Button = builder.get_object("button").unwrap();
    let scale: gtk::Scale = builder.get_object("scale").unwrap();

    button.connect_clicked(|_| {
        println!("Clicked!");
    });

    scale.connect_value_changed(|_| {
        println!("scale changed");
    });

    window.show_all();

    gtk::main();
}
