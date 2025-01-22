use glib::subclass::InitializingObject;
use gtk::{glib::{self}, subclass::prelude::*, CompositeTemplate, Label};
use std::{boxed::Box, error::Error};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/ambertia/rust-calculator-gui/calculator.ui")]
pub struct Window {
    #[template_child]
    pub operand_buffer_label: TemplateChild<Label>,
    #[template_child]
    pub operand_input_label: TemplateChild<Label>,
    #[template_child]
    pub operation_label: TemplateChild<Label>,
}

impl Window {
    // Clear all labels
    pub fn clear(&self) {
        self.operand_buffer_label.set_label("");
        self.operand_input_label.set_label("");
        self.operation_label.set_label("");
    }

    // Process a special action dispatch
    pub fn process_special(&self, parameter: &str) {
        let input: &str = &self.operand_input_label.label();
        match parameter {
            "C" => self.clear(),
            "D" => {
                let label_text = self.operand_input_label.label();
                self.operand_input_label.set_label(label_text.split_at(label_text.len() - 1).0);
            },
            "-" => match input.strip_prefix("-") {
                Some(s) => self.operand_input_label.set_label(s),
                None => self.operand_input_label.set_label(&("-".to_owned() + input))
            },
            "." => if !input.contains(".") {
                self.operand_input_label.set_label(&(input.to_owned() + "."))
            },
            "=" => match self.calculate() {
                Ok(d) => { self.clear(); self.operand_buffer_label.set_label(&d.to_string()); },
                Err(_) => { self.clear(); self.operand_buffer_label.set_label("ERR"); }
            },
            _ => {}
        }
    }

    fn calculate(&self) -> Result<Decimal, Box<dyn Error>> {
        // Get the operation label
        let operation: &str = &self.operation_label.label();

        // Get the first operand from its label
        let input  = match Decimal::from_str(&self.operand_input_label.label()) {
            Ok(d) => d,
            Err(e) => return Err(Box::new(e)),
        };

        // Get the second operand from its label, but don't abort on error if the operation doesn't need this operand
        let buffer = match Decimal::from_str(&self.operand_buffer_label.label()) {
            Ok(d) => d,
            Err(_) if "√".contains(operation) => dec!(0.),
            Err(e) => return Err(Box::new(e)),
        };

        // Conduct calculation
        return match operation {
            "+" => Ok(buffer + input),
            "-" => Ok(buffer - input),
            "*" => Ok(buffer * input),
            "÷" => Ok(buffer / input),
            "^" => Ok(buffer.checked_powd(input).unwrap()),
            "√" => Ok(input.sqrt().unwrap()),
            _ => Err(Box::<dyn Error>::from("Unknown operation")),
        }
    }
}

// Central trait for subclassing an object
#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "RustCalculatorGTKWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {

        // Call "constructed" on parent
        self.parent_constructed();

        // Add actions
        self.obj().setup_actions();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}