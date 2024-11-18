use glib::subclass::InitializingObject;
use gtk::{glib, Box, subclass::prelude::*, CompositeTemplate};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/ambertia/rust-calculator-gui/calculator.ui")]
pub struct Window {
    #[template_child]
    pub box_root: TemplateChild<Box>,
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