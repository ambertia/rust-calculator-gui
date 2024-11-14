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
    // TODO Calculator input & results rectangle

    // TODO Calculator buttons
}
