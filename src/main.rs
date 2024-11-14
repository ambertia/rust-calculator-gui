use gtk::{glib, prelude::*, Application};

const APP_ID: &str = "com.github.ambertia.rust-calculator-gui";

fn main() -> glib::ExitCode {
    // Create the GTK application
    let app = Application::builder().application_id(APP_ID).build();

    // Build the UI when the app activates
    app.connect_activate(build_ui);

    // Run the application and return the error code when it ends
    app.run()
}

fn build_ui(app: &Application) {
    // Vertical box to represent the root pane
    let box_root = gtk::Box::new(gtk::Orientation::Vertical, 5);

    // TODO Calculator input & results rectangle

    // TODO Calculator buttons

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Rust Calculator")
        .child(&box_root)
        .build();

    window.present();
}
