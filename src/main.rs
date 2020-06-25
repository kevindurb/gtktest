extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow, Button, Scale};

fn main() {
    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(350, 70);

        let container = gtk::Box::new(gtk::Orientation::Vertical, 16);

        let button = Button::new_with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });
        container.add(&button);

        let scale = Scale::new_with_range(gtk::Orientation::Horizontal, 0.0, 10.0, 1.0);
        scale.connect_value_changed(|_| {
            println!("new value {}", scale.getValue());
        });
        container.add(&scale);

        window.add(&container);

        window.show_all();
    });

    application.run(&[]);
}
