mod equations;
mod window;

use gtk::{gdk::Display, gio::{self, ActionEntry}, glib, prelude::*, Application, CssProvider};
use window::Window;

const APP_ID: &str = "com.github.ambertia.rust-calculator-gui";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("calculator.gresource")
        .expect("Failed to load calculator gresource");

    // Create the GTK application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|app| {
        set_accelerators(app);
        load_css();
    });

    // Build the UI when the app activates
    app.connect_activate(build_ui);

    // Run the application and return the error code when it ends
    app.run()
}

fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);

    // Add action and accelerator to close the window
    let close = ActionEntry::builder("close")
        .activate(|window: &window::Window, _, _| {
            window.close()
        })
        .build();
    window.add_action_entries([close]);
    app.set_accels_for_action("win.close", &["<Ctrl>w"]);

    // Connect the window height change to a closure that moves in a CSS provider
    // CSS provider does math on the window height to determine font size
    /* This method technically works but if the font is made too large by default,
    the window gets "stuck" to certain height ranges and can't be shrunk back down.
    My guess is that the font infringes on the label padding or margins or whatnot
    in a way that prevents the height from being scaled back down*/
    let font_scaling_provider = CssProvider::new();
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display"),
        &font_scaling_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    window.connect_default_height_notify(move |window: &Window| {
        let target_size = f64::from(window.height()) / 300.0 * 24.0;
        println!("target_size: {0:.1}px", target_size);
        font_scaling_provider.load_from_string(format!("window {{ font-size: {0:.1}px; }}", target_size).as_str());
    });

    window.present();
}

fn load_css() {
    // Load the CSS file
    let provider = CssProvider::new();
    provider.load_from_resource("/com/github/ambertia/rust-calculator-gui/style.css");

    // Add provider to the default display
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
}

fn set_accelerators(app: &Application) {
    // Key accelerators for digit dispatches
    app.set_accels_for_action("win.digit(0)", &["0"]);
    app.set_accels_for_action("win.digit(1)", &["1"]);
    app.set_accels_for_action("win.digit(2)", &["2"]);
    app.set_accels_for_action("win.digit(3)", &["3"]);
    app.set_accels_for_action("win.digit(4)", &["4"]);
    app.set_accels_for_action("win.digit(5)", &["5"]);
    app.set_accels_for_action("win.digit(6)", &["6"]);
    app.set_accels_for_action("win.digit(7)", &["7"]);
    app.set_accels_for_action("win.digit(8)", &["8"]);
    app.set_accels_for_action("win.digit(9)", &["9"]);

    // Key accelerators for operation dispatches
    app.set_accels_for_action("win.operation('+')", &["plus"]);
    app.set_accels_for_action("win.operation('-')", &["minus"]);
    app.set_accels_for_action("win.operation('*')", &["asterisk"]);
    app.set_accels_for_action("win.operation('÷')", &["slash"]);
    app.set_accels_for_action("win.operation('^')", &["asciicircum"]);
    app.set_accels_for_action("win.operation('√')", &["r"]);

    // Key accelerators for special dispatches
    app.set_accels_for_action("win.special('C')", &["c", "<Shift>c", "Escape"]);
    app.set_accels_for_action("win.special('-')", &["n", "<Shift>n", "underscore"]);
    app.set_accels_for_action("win.special('.')", &["period"]);
    app.set_accels_for_action("win.special('=')", &["equal", "Return"]);
    app.set_accels_for_action("win.special('D')", &["BackSpace"])
}