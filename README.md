# Rustic Time Sheets

**Rustic Time Sheets** is a desktop application designed to track projects, tasks, and price codes, and calculate earnings for completed tasks. The application leverages [Slint](https://slint.rs/) for its user interface and [Rust](https://www.rust-lang.org/) for the backend, combining modern UI design with robust performance.

## Features

- **Track Tasks and Projects**: Manage and monitor various projects and tasks.
- **Price Codes**: Associate tasks with custom price codes to calculate earnings.
- **CSV Export**: Generate and export time sheets to CSV format for record-keeping.

## Installation and Usage

1. **Install Rust**: Follow the [Rust getting started guide](https://www.rust-lang.org/learn/get-started) to install the Rust toolchain, including `rustc` and `cargo`.
2. **Clone the Repository**:
    ```sh
    git clone https://github.com/gb270/rustic-time-sheets.git
    cd rustic-time-sheets
    ```
3. **Build the Application**:
    ```sh
    cargo build
    ```
4. **Run the Application**:
    ```sh
    cargo run
    ```

For development, we recommend using an IDE with support for Slint files. You can load the project in [Visual Studio Code](https://code.visualstudio.com) and install the [Slint extension](https://marketplace.visualstudio.com/items?itemName=Slint.slint) for enhanced development experience.

## Planned Improvements

- [ ] Add functionality for custom project names.
- [ ] Implement GUI-based custom price codes.
- [ ] Allow simultaneous task execution.
- [ ] Introduce persistence to retain data across sessions and avoid constant CSV regeneration.

## Contributing

Feel free to contribute to the project by submitting issues or pull requests.

## License

This project is licensed under the [MIT License](LICENSE) - see the [LICENSE](LICENSE) file for details.
