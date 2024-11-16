mod window;

use gtk::{gio, glib, prelude::*, Application};
use window::Window;

const APP_ID: &str = "com.github.ambertia.rust-calculator-gui";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("calculator.gresource")
        .expect("Failed to load calculator gresource");

    // Create the GTK application
    let app = Application::builder().application_id(APP_ID).build();

    // Build the UI when the app activates
    app.connect_activate(build_ui);

    // Run the application and return the error code when it ends
    app.run()
}

fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}

// Return a ButtonBuilder for a calculator button with the common properties already set
fn grid_button_builder_factory() -> gtk::builders::ButtonBuilder {
    gtk::Button::builder()
        .hexpand(true)
        .vexpand(true)
}