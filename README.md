🚧 Under construction 🚧
# rust-calculator-gui

This project explores creating desktop applications using Rust and GTK4. Moving beyond JavaFX, I chose GTK4 with the [GTK Rust bindings](https://gtk-rs.org/) to learn new tools and techniques. This was my first multi-day project using Rust and GTK, and I’m proud of the result. I had to heavily utilize the documentation for [GTK4](https://docs.gtk.org/gtk4/getting_started.html) and [the rust bindings](https://docs.rs/gtk4/latest/gtk4/). Besides these, there were two key resources that were extremely useful in creating this project:
1. [A Half-Hour to Learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust) by Amos (fasterthanlime), which provided a concise guide to Rust's syntax and quirks.
2. [GUI Development with Rust and GTK4](https://gtk-rs.org/gtk4-rs/git/book/introduction.html), the official GTK4 book, which detailed how to implement the application functionality and manage GTK’s unique integration with Rust.

## Core Moving Parts

```
.
├─ src
│   ├─ resources: Directory containing files compiled into GTK GResources.
│   │   ├─ calculator.ui: XML file defining the application's UI and layout.
│   │   ├─ resources.gresource.xml: Specifies files to include in GResources.
│   │   └─ style.css: Provides standardized styles (limited to supported CSS features).
│   └─ window: Custom module "subclassing" `gtk::ApplicationWindow`. Following GTK
│       │  Rust conventions, the module is split into `mod.rs` and `imp.rs`.
│       ├─ mod.rs: Core implementation, including a wrapper struct using GTK’s
│       │    has-a inheritance model. Contains essential functions like `Window::new()`
│       │    and `Window::setup_actions()`, and is imported into `main.rs`.
│       └─ imp.rs: Implements module functionality. Defines behaviors extended by
│            the module, such as `impl ApplicationWindowImpl for Window {}`. Handles
│            class and object initialization and instance methods.
└─ build.rs: Cargo build script specifying that `glib_build_tools` should compile
       files listed in `resources.gresource.xml` into GResources.
```

## Interesting Learning Topics
### Inheritance in Rust / Traits
GTK is an object-oriented GUI library that is heavily dependent on traditional subclassing / inheritance, a.k.a **is-a** inheritance. To my understanding Rust doesn't really have this kind of inheritance; inheritance in Rust is instead based on composition through Traits, which work similarly to interfaces in other languages like Java (also known as **has-a** inheritance). There's a good deal of boilerplate code in Rust GTK projects that serves to emulate object-oriented functionality; one such example is the `glib::wrapper! { pub struct Window(ObjectSubclass<imp::Window>) }` declaration in `mod.rs`, which defines all the `GObjects` that the custom subclass "extends" or "implements" in the base framework. Then, in `imp.rs`, there are a bunch of different sections of code which implement traits on the custom subclass. For example, in other languages an extended application window might be declared with `Window extends ApplicationWindow`, or in other words, Window *is-a* ApplicationWindow. In gtk-rs you instead write `impl ApplicationWindowImpl for Window {}`, or in other words Window *has-a* ApplicationWindowImplementation. With the blocks empty, the function definitions and implementations are all pulled up from the base class. There are two particular `impl` blocks that are special when subclassing a GTK object in Rust. The first is `ObjectSubclass`, which defines some of the meta-characteristics of the class as a whole and how it should be built (for example, binding struct members to GTK template UI elements). The second is `ObjectImpl`, containing a function `constructed`. This function is called after the class is instantiated; in my case, it then calls a function to set up the `gtk::ActionMap` for the `Window`.
### Handling every edge case with `Option` and `Result`
One of the most interesting features of Rust that contrasts it with other languages, and indeed one of the reasons I was interested in learning it, are these two special enums. `Option<T>` can contain either an object of type `T` or `None`, while `Result<T,E>` can contain either an object of type `T` or an error of type `E`. Many functions in Rust that have the potential to fail or have weird edge cases return Options or Results; in this way, errors are not handled by breaking the flow of control but rather by being an integral part of it. An example where I used this myself in a non-trivial fashion are the functions `process_special(&self, &str)` and `calculate(&self)` in the `impl Window` block of `imp.rs`. When `calculate` is called in `process_special`, it returns a `Result<f64, Box<dyn Error>>` which the calling function then `match`es, either displaying the result if it returns `Ok(f64)` or displaying "ERR" if it returns a `Box<dyn Error>`. This way, there are no exceptions to catch when there is a `ParseFloatError` or an invalid operation custom error (`Box::<dyn Error>::from(&str)`); the program instead handles it as a matter of course. There are other places, particularly in the API where a developer could be using something wrong, where enums are collapsed using `expect(&str)`. In this case, the developer is *explicitly specifying* that the program should panic with a runtime error if an `Option` turns out to be `None` or if a `Result` turns out to be an `Error`.