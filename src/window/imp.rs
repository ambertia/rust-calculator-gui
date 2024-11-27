use glib::subclass::InitializingObject;
use gtk::{glib::{self, Propagation}, prelude::{GtkWindowExt, WidgetExt}, subclass::prelude::*, Box, CompositeTemplate, ShortcutController};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/ambertia/rust-calculator-gui/calculator.ui", allow_template_child_without_attribute)]
pub struct Window {
    pub box_root: TemplateChild<Box>,
    pub operand_buffer_label: TemplateChild<Label>,
    pub operand_input_label: TemplateChild<Label>,
    pub operation_label: TemplateChild<Label>,

    // State properties for the calculator
    pub buffer_operand: f64,
    pub input_operand: f64,
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