# rust-calculator-gui

This project explores creating desktop applications using Rust and GTK4. Moving beyond JavaFX, I chose GTK4 with the [GTK Rust bindings](https://gtk-rs.org/) to learn new tools and techniques. This was my first multi-day project using Rust and GTK, and I’m proud of the result. I had to heavily utilize the documentation for [GTK4](https://docs.gtk.org/gtk4/getting_started.html) and [the rust bindings](https://docs.rs/gtk4/latest/gtk4/). Besides these, there were two key resources that were extremely useful in creating this project:
1. [A Half-Hour to Learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust) by Amos (fasterthanlime), which provided a concise guide to Rust's syntax and quirks.
2. [GUI Development with Rust and GTK4](https://gtk-rs.org/gtk4-rs/git/book/introduction.html), the official GTK4 book, which detailed how to implement functionality and manage GTK’s unique integration with Rust.

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