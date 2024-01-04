use std::cell::Cell;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("ON")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let status = Cell::new(false);
    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        if status.get() {
            button.set_label("ON");
            status.set(false)
        } else {
            button.set_label("OFF");
            status.set(true)
        }
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    // Present window
    window.present();
}