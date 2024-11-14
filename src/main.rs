use gtk::{glib, prelude::*, Application, ApplicationWindow};

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
    let top_label_result = gtk::Label::builder().hexpand(true).build();
    let top_label_operation = gtk::Label::builder().width_request(100).build();
    let top_entry = gtk::Entry::builder().hexpand(true).build();

    // Horizontal box to represent the results pane
    let box_top = gtk::CenterBox::builder()
        .vexpand(true)
        .start_widget(&top_label_result)
        .center_widget(&top_label_operation)
        .end_widget(&top_entry)
        .build();
    box_root.append(&box_top);

    // TODO Calculator buttons

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Rust Calculator")
        .child(&box_root)
        .build();

    window.present();
}
