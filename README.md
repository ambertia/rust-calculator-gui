# rust-calculator-gui

This project explores creating desktop applications using Rust and GTK4. Moving beyond JavaFX, I chose GTK4 with the [GTK Rust bindings](https://gtk-rs.org/) to learn new tools and techniques. This was my first multi-day project using Rust and GTK, and I’m proud of the result. I had to heavily utilize the documentation for [GTK4](https://docs.gtk.org/gtk4/getting_started.html) and [the rust bindings](https://docs.rs/gtk4/latest/gtk4/). Besides these, there were two key resources that were extremely useful in creating this project:
1. The famous article [A Half-Hour to Learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust) by Amos (fasterthanlime), which provided a concise guide to Rust's syntax and quirks.
2. [GUI Development with Rust and GTK4](https://gtk-rs.org/gtk4-rs/git/book/introduction.html), the official GTK4 book, which details how to create applications and manage GTK’s unique integration with Rust.

## Core Moving Parts

```
.
├─ src
│   ├─ resources: Directory containing files compiled into GTK GResources.
│   │   ├─ calculator.ui: XML file defining the application's UI and layout.
│   │   ├─ resources.gresource.xml: Specifies files to include in GResources.
│   │   └─ style.css: Provides standardized styles (limited to supported CSS features).
│   └─ window: Custom module "subclassing" gtk::ApplicationWindow. Following GTK
│       │  Rust conventions, the module is split into mod.rs and imp.rs.
│       ├─ mod.rs: Core implementation, including a wrapper struct using GTK’s
│       │    has-a inheritance model. Contains essential functions like Window::new()
│       │    and Window::setup_actions(), and is imported into main.rs.
│       └─ imp.rs: Implements module functionality. Defines behaviors extended by
│            the module, such as impl ApplicationWindowImpl for Window {}. Handles
│            class and object initialization and instance methods.
└─ build.rs: Cargo build script specifying that glib_build_tools should compile
       files listed in resources.gresource.xml into GResources.

```
## How It Works
### Application Startup
Application startup is handled in `main()`. The statements in `main` have the following functions:
1. Load assets from GResources, which are files compiled / bundled inside of GTK applications. For this project, the only GResources were a `.xml` file describing the UI layout and element properties, and a `.css` style sheet.
2. Create the root application object.
3. Set accelerators (application-wide keybindings) and load the style sheet on startup.
4. Build the UI (create an actual window for the application) on activation.
5. Run the application (hand off processing to GTK's runtime so that it can start handling the event loop).
### Flow of Control
Every button in the calculator's UI as well as the majority of the key bindings dispatch one of three custom `Action`s defined as `ActionEntry`s in `mod.rs/setup_actions()`.
1. `dispatch_digit` handles adding digits to the data entry label when a digit button or key is pressed. The label text itself is used as the storage for this value, as it's more convenient to modify a string in-situ and then worry about parsing it once at the end rather than constantly convert back and forth; additionally, using the label text avoids having to create an extra state variable to store the exact same information.
2. `dispatch_operation` handles characters representing the calculator's available mathematical operations. If the buffer (upper label) is not empty when an operation requiring two operands is dispatched, the value in the data entry will be moved up into the buffer and the data entry cleared.
3. `dispatch_special` handles more abstract calculator functions like clearing, backspace / character deletion, the equals button, and so on.
### Calculator Processing
Two instance methods associated with the custom `Window` subclass handle the meat and potatoes of the calculator's functionality:
1. `process_special()` takes in a special dispatch character (technically a `&str`) and will perform various changes in state of the calculator depending on its value; e.g., `=` will call `calculate()`.
2. `calculate()` attempts to parse doubles from one or both operand labels depending on the operation. It will then calculate the result using a `match` block and return it to be displayed.

## Things I'd Like to Improve
- The output currently shows a fixed number of decimal places in its output, which is cumbersome and restrictive.
  - Integer operations will display decimal points. For example, `2 + 3` will output `5.00000`. While this is technically correct, it isn't very graceful; operations involving integers (excepting things like roots or division) should display integer results without any decimal places.
  - Decimal operations do not have a very graceful output. For example, `1.5 - .25` will output `1.25000`. While this is again technically correct, ideally it would display only `1.25`.
  - Decimal operations are restricted to the hundred-thousandths place at smallest due to the hard-coded number of displayed decimal places.
    - This will result in arithmetic inaccuracies due to truncation of significant digits for small-scale (or very precise) floating point operations.
- The flow of the user interface is not particularly friendly. The manipulation of the data entry could be confusing to some users, and the fact that the previous result will remain displayed in the buffer label when a single-operand operation such as square root is selected may be unclear to users. This is worsened by the square root operation accepting *either* the buffer label value or data entry label, with preference for the latter.
  - If the user selects an operation before supplying any operand, there isn't an elegant way to move the first operand from the data entry into the buffer except by selecting the operation again.

## Things I Learned
The following is a non-exhaustive list of things that I learned working on this project. If I went into too much detail I'd be writing a book, so I tried to focus on some of the key points.
### Rust's compiler / the rust-analyzer extension
I greatly appreciate how helpful Rust's compiler errors are, as well as how comprehensively the rust-analyzer VSCode extension is able to detect problems. Rust is very different from other languages I've worked with and has some pieces of syntax, such as borrowing, that I haven't particularly used before. If it wasn't for how helpful the software was to me while I was programming, it would've been much more difficult for me to finish this project!
### Rust's Borrowing System
While I'm still far from an expert at understanding all the intricacies of Rust's lifetime and borrowing systems, I respect the memory-safe approach Rust takes. Even though it can force the programmer to design things a bit differently than you might in other languages, the immutability by default and the borrowing system force you as a programmer to think very explicitly about the who, what, when, where, why, and how of your variables and data structures.
### Inheritance in Rust / Traits
GTK is an object-oriented GUI library that is heavily dependent on traditional subclassing / inheritance, a.k.a **is-a** inheritance. To my understanding Rust doesn't really have this kind of inheritance; inheritance in Rust is instead based on composition through Traits, which work similarly to interfaces in other languages like Java (also known as **has-a** inheritance). There's a good deal of boilerplate code in Rust GTK projects that serves to emulate object-oriented functionality; one such example is the `glib::wrapper! { pub struct Window(ObjectSubclass<imp::Window>) }` declaration in `mod.rs`, which defines all the `GObjects` that the custom subclass "extends" or "implements" in the base framework. Then, in `imp.rs`, there are a bunch of different sections of code which implement traits on the custom subclass. For example, in other languages an extended application window might be declared with `Window extends ApplicationWindow`, or in other words, Window *is-a* ApplicationWindow. In gtk-rs you instead include an `impl ApplicationWindowImpl for Window {}` block in the subclass, or in other words specify that Window *has-a* ApplicationWindowImplementation. With the blocks empty, the function definitions and implementations are all pulled up from the base class. There are two particular `impl` blocks that are special when subclassing a GTK object in Rust. The first is `ObjectSubclass`, which defines some of the meta-characteristics of the class as a whole and how it should be built (for example, binding struct members to GTK template UI elements). The second is `ObjectImpl`, containing a function `constructed`. This function is called after the class is instantiated; in my case, it then calls a function to set up the `gtk::ActionMap` for the `Window`.
### Handling every edge case with `Option` and `Result`
One of the most interesting features of Rust in my opinion is these two special enums; in fact, learning about their existence was one of the very reasons I wanted to try out Rust. `Option<T>` can contain either an object of type `T` or `None`, while `Result<T,E>` can contain either an object of type `T` or an error of type `E`. Many functions in Rust that have the potential to fail or have weird edge cases return Options or Results; in this way, errors are not handled by breaking the flow of control but rather by being an integral part of it. An example where I used this myself in a non-trivial fashion are the functions `process_special(&self, &str)` and `calculate(&self)` in the `impl Window` block of `imp.rs`. When `calculate` is called in `process_special`, it returns a `Result<f64, Box<dyn Error>>` which the calling function then `match`es, either displaying the result if it returns `Ok(f64)` or displaying "ERR" if it returns a `Box<dyn Error>`. This way, there are no exceptions to catch when there is a `ParseFloatError` or an invalid operation custom error (`Box::<dyn Error>::from(&str)`); the program instead handles it as a matter of course. There are other places, particularly in the API where a developer could be using something wrong, where enums are collapsed using `expect(&str)`. In this case, the developer is *explicitly specifying* that the program should panic with a runtime error if an `Option` turns out to be `None` or if a `Result` turns out to be an `Error`. This structuring forces the programmer to handle any unexpected outcomes as gracefully as possible, and otherwise to explicitly acknowledge every single place where things can go wrong. In fact, it can make sense to work out a skeleton of the application's functionality while using `unwrap()` or `expect()` to unpack `Option`s and `Result`s as needed at first, and then go back through all of these function calls later to find out which ones could be replaced with an elegant means of handling the problem.
