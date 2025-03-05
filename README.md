# Dioxus Todo App with Material Design

A minimalist Todo application built with Rust and the Dioxus framework, featuring a clean Material Design-inspired UI.

![Dioxus Todo App](https://github.com/DioxusLabs/dioxus/raw/master/assets/dioxus_logo.png)

## Features

- ✅ Create, complete, and delete tasks
- 🎨 Material Design-inspired user interface
- 🦀 Built with Rust for safety and performance
- ⚡ Reactive UI with Dioxus framework
- 📱 Responsive layout suitable for all devices

## Tech Stack

- **Rust** - Safe, concurrent programming language
- **Dioxus** - Portable, performant framework for building cross-platform user interfaces
- **Material Design** - Google's design system implemented with custom CSS

## Development

This project follows a structured organization with:
- `assets/styling` - Contains the CSS styles
- `src/components` - Reusable UI components
- `src/views` - Application views and pages

### Running the App

To run the application, first ensure you have Rust and the Dioxus CLI installed:

```bash
# Install Rust (if you haven't already)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Dioxus CLI
cargo install dioxus-cli
```

Then, from the project directory, run:

```bash
# Run the app in development mode
dx serve --platform web
```

The application will be available at: http://localhost:8080

### Building for Production

To build for production, run:

```bash
dx build --release --platform web
```

The build artifacts will be stored in the `dist/` directory.

## Project Structure

```
demo-dioxus/
├── assets/
│   └── styling/
│       └── main.css        # Material Design-inspired styles
├── src/
│   ├── components/
│   │   ├── mod.rs          # Components module
│   │   ├── navbar.rs       # Navbar component
│   │   └── todo.rs         # Todo item component
│   ├── views/
│   │   ├── mod.rs          # Views module
│   │   └── todo_list.rs    # TodoList view with main functionality
│   └── main.rs             # Application entry point
├── Cargo.toml              # Rust dependencies
└── README.md               # This readme
```

## Features

### Todo Management
- Add new tasks with the input field
- Mark tasks as completed by clicking the checkbox
- Delete tasks by clicking the delete button
- Clear all completed tasks with one click

### UI Components
- Clean, minimalist Material Design interface
- Responsive layout that works on all screen sizes
- Task counter showing the number of remaining tasks
- Empty state when no tasks are present

## License

This project is open source and available under the MIT License.

## Acknowledgments

- [Dioxus](https://dioxuslabs.com/) - for the amazing Rust UI framework
- [Material Design](https://material.io/design) - for design inspiration

