use glib::subclass::InitializingObject;
use gtk::{glib::{self, Propagation}, prelude::{GtkWindowExt, WidgetExt}, subclass::prelude::*, Box, CompositeTemplate, ShortcutController};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/ambertia/rust-calculator-gui/calculator.ui")]
pub struct Window {
    #[template_child]
    pub box_root: TemplateChild<Box>,

    // State properties for the calculator
    pub buffer_operand: f64,
    pub right_operand: f64,
    pub operation: String,
}

// Central trait for subclassing an object
#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "RustCalculatorGTKWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl Window {

    // Primary callback used by the calculator grid buttons
    // Try to print the label's text to stdout, or None if there is no text
    #[template_callback]
    fn dispatch_grid_click(button: &gtk::Button) {
        let button_label = gtk::prelude::ButtonExt::label(button);
        match button_label {
            Some(s) => println!("{s}"),
            None => println!("None")
        }
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Add keybindings to the window
        let event_controller = gtk::EventControllerKey::new();
        event_controller.connect_key_pressed(|_, key, _, _| {
            match key {
                _ => { println!("key pressed"); Propagation::Stop },
            }
        });
        gtk::prelude::WidgetExt::add_controller(self.default_widget(), event_controller);

        // Call "constructed" on parent
        self.parent_constructed();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}