mod imp;

use glib::Object;
use gtk::{gio::{self, ActionEntry}, glib, prelude::{ActionMapExtManual, StaticVariantType}, subclass::prelude::ObjectSubclassIsExt, Application};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    fn setup_actions(&self) {

        // Action when a digit is dispatched
        let dispatch_digit = ActionEntry::builder("digit")
            .parameter_type(Some(&i32::static_variant_type()))
            .activate(move |window: &Self, _action, parameter| {
                // Fetch the parameter as i32
                let parameter = parameter
                    .expect("Could not get parameter")
                    .get::<i32>()
                    .expect("Digit parameter should be i32");
                // Core callback functionality
                window.imp().operand_input_label.set_label(&format!("{parameter}"));
            })
            .build();

        // Action when an operation is dispatched
        let dispatch_operation = ActionEntry::builder("operation")
            .parameter_type(Some(&String::static_variant_type()))
            .activate(move |window: &Self, _action, parameter| {
                let parameter = parameter
                    .expect("Could not get parameter")
                    .get::<String>()
                    .expect("Operation parameter should be String");
                // Core callback functionality
                window.imp().operation_label.set_label(parameter.as_str());
            })
            .build();

        // Action for special dispatches (clear, negative, decimal point, equals)
        let dispatch_special = ActionEntry::builder("special")
            .parameter_type(Some(&String::static_variant_type()))
            .activate(move |window: &Self, _action, parameter| {
                let parameter = parameter
                    .expect("Could not get parameter")
                    .get::<String>()
                    .expect("Special parameter should be String");
                // Core callback functionality
                window.imp().operand_buffer_label.set_label(parameter.as_str());
                if parameter == "C" { window.imp().clear() };
            })
            .build();

        self.add_action_entries([dispatch_digit, dispatch_operation, dispatch_special]);
    }
}