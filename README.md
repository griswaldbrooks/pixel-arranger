# pixel arranger

pixel arranger is a simple multi image viewer built using Rust and Egui. It allows users to view, arrange, zoom, and manage multiple images on a canvas.

## Features

- Load and display multiple images on a canvas
- Move images around the canvas
- Zoom in and out of images using the mouse scroll wheel
- Add and remove images using a context menu
- Save and load the application state, including image positions and window dimensions

## Installation

Make sure you have [Rust and Cargo installed](https://www.rust-lang.org/tools/install) on your system.

1. Clone the repository:

```
git clone https://github.com/yourusername/pixel-arranger.git
cd pixel-arranger
```

2. Build and run the application:

```
cargo build --release
cargo run --release
```

## Usage

- To add images, right-click on the canvas and select "Add Image" from the context menu. Choose the images you want to add from the file picker dialog.
- To move an image, click and drag it to the desired position.
- To zoom in or out of an image, hover over it and scroll up or down with the mouse wheel.
- To remove an image, right-click on it and select "Remove Image" from the context menu.
- The application state is automatically saved to a JSON file when you close the application and loaded when you start the application.

## Linting
To enforce code formatting and create documentation in a Rust project, you can use the following tools:

1. **Rustfmt**: Rustfmt is the official code formatter for Rust. It enforces consistent code style across the project. To install Rustfmt, run:

```
rustup component add rustfmt
```

To format your code, navigate to the project directory and run:

```
cargo fmt
```

This command will format all the Rust source files in your project according to the default configuration.

2. **Clippy**: Clippy is a collection of lints that can help you catch common mistakes and improve your Rust code. To install Clippy, run:

```
rustup component add clippy
```

To run Clippy on your project, navigate to the project directory and run:

```
cargo clippy
```

This command will show you any warnings or suggestions for improving your code.

3. **Rustdoc**: Rustdoc is the documentation generator for Rust. It generates documentation directly from your source code comments. To create documentation for your project, navigate to the project directory and run:

```
cargo doc --no-deps --open
```

This command will generate the documentation and open it in your default web browser. The `--no-deps` flag tells Rustdoc not to generate documentation for your project's dependencies.

Make sure to write descriptive comments using the triple slashes `///` above your functions, structs, and modules to provide helpful information in the generated documentation.
