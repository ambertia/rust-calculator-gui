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

    // Top box to display the result, current operation, and user-typed argument
    let top_label_result = gtk::Label::builder().hexpand(true).label("result").build();
    let top_label_operation = gtk::Label::builder().width_request(100).label("*").build();
    let top_entry = gtk::Entry::builder().hexpand(true).build();

    // Horizontal box to represent the results pane
    let box_top = gtk::CenterBox::builder()
        .height_request(120)
        .start_widget(&top_label_result)
        .center_widget(&top_label_operation)
        .end_widget(&top_entry)
        .build();
    box_root.append(&box_top);

    // Grid for the calculator buttons
    let button_grid = gtk::Grid::new();
    button_grid.set_row_spacing(5);
    button_grid.set_column_spacing(5);
    button_grid.set_margin_top(5);
    button_grid.set_margin_bottom(5);
    button_grid.set_margin_start(5);
    button_grid.set_margin_end(5);
    box_root.append(&button_grid);

    // Add all of the intended buttons to the grid, using the builder factory for consistent properties
    // Button row 0 (top)
    button_grid.attach(&grid_button_builder_factory().label("C").build(), 0, 0, 1, 1);
    button_grid.attach(&grid_button_builder_factory().label("√").build(), 1, 0, 1, 1);
    button_grid.attach(&grid_button_builder_factory().label("^").build(), 2, 0, 1, 1);
    button_grid.attach(&grid_button_builder_factory().label("÷").build(), 3, 0, 1, 1);

    // Row 1
    button_grid.attach(&grid_button_builder_factory().label("7").build(), 0, 1, 1, 1);
    button_grid.attach(&grid_button_builder_factory().label("8").build(), 1, 1, 1, 1);
    button_grid.attach(&grid_button_builder_factory().label("9").build(), 2, 1, 1, 1);
    button_grid.attach(&grid_button_builder_factory().label("*").build(), 3, 1, 1, 1);

    // Row 2
    button_grid.attach(&grid_button_builder_factory().label("4").build(), 0, 2, 1, 1);
    button_grid.attach(&grid_button_builder_factory().label("5").build(), 1, 2, 1, 1);
    button_grid.attach(&grid_button_builder_factory().label("6").build(), 2, 2, 1, 1);
    button_grid.attach(&grid_button_builder_factory().label("-").build(), 3, 2, 1, 1);

    // Row 3
    button_grid.attach(&grid_button_builder_factory().label("1").build(), 0, 3, 1, 1);
    button_grid.attach(&grid_button_builder_factory().label("2").build(), 1, 3, 1, 1);
    button_grid.attach(&grid_button_builder_factory().label("3").build(), 2, 3, 1, 1);
    button_grid.attach(&grid_button_builder_factory().label("+").build(), 3, 3, 1, 1);

    // Row 4 (bottom)
    button_grid.attach(&grid_button_builder_factory().label("(-)").build(), 0, 4, 1, 1);
    button_grid.attach(&grid_button_builder_factory().label("0").build(), 1, 4, 1, 1);
    button_grid.attach(&grid_button_builder_factory().label(".").build(), 2, 4, 1, 1);
    button_grid.attach(&grid_button_builder_factory().label("=").build(), 3, 4, 1, 1);

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Rust Calculator")
        .child(&box_root)
        .build();

    window.present();
}

// Return a ButtonBuilder for a calculator button with the common properties already set
fn grid_button_builder_factory() -> gtk::builders::ButtonBuilder {
    gtk::Button::builder()
        .hexpand(true)
        .vexpand(true)
}