# Walter Core DevTools

Walter Core DevTools is a collection of tools for the Walrus file storage network. It includes the following features:

| Feature | Description |  
|---------|-------------|  
| 🔄 Sharding | Advanced file sharding support for handling large files |  
| 🔒 Encryption | End-to-end file encryption and decryption |  
| ✅ Verification | Robust file integrity verification system |  
| 📦 Auto-Update | Automated Walrus binary updates |  
| ⏰ Epoch Extension | Extend your storage duration with Epoch Extender |  
| 🔄 Migration | One-click migration from other IPFS providers |  
| 📌 HTTP Pinning | Persistent file pinning via HTTP |   

# Walter UI

Walter UI is a **terminal-based** user interface for managing and monitoring your Rust projects. It leverages the power of `crossterm` for terminal handling, `ratatui` for building rich terminal UIs, and `reqwest` for making HTTP requests. This project is part of the larger Walter system, which includes core functionalities provided by `walter-core`.

## Features

### 1. Splash Screen
- **Navigation**: Press `Enter` to proceed to the Dashboard.
- **Scrollbar**: Automatically initializes if user blobs are present.

### 2. Dashboard
- **Quit Application**: Press `q` to initiate quit, then `y` to confirm or `n` to cancel.
- **Navigation**: 
    - Press `1` to switch to the Dashboard screen.
    - Press `2` to switch to the Updater screen.
    - Use `Up` and `Down` arrow keys to navigate through rows.
- **Current Screen**: Displays the current screen (Dashboard or Updater).

### 3. Updater
- **Functionality**: (Commented out in the current version)
    - Intended to create a `walrus.json` file with system information.
    - Press `Enter` to execute the update process.

## Dependencies

- **crossterm**: For handling terminal I/O.
- **ratatui**: For building the terminal UI.
- **reqwest**: For making HTTP requests.
- **serde**: For serializing and deserializing data.
- **serde_json**: For working with JSON data.
- **tokio**: For asynchronous programming.
- **walter-core**: Core functionalities shared across the Walter system.

## Getting Started

### Prerequisites

- Rust and Cargo installed on your system.

### 📦 Installation

1. Clone the repository:
        ```
        git clone https://github.com/EthIndia2024IITR/walter-ui ;
        cd walter-ui
        ```

2. Build the project:
        ```
        cargo build
        ```

3. Run the project:
        ```
        cargo run
        ```

## License

This project is licensed under the terms of the license found in the `LICENSE` file in the root directory of this source tree.

## Contributing

Contributions are welcome! Please read the `CONTRIBUTING.md` file for guidelines on how to contribute to this project.

## Acknowledgements

- Thanks to the developers of `crossterm`, `ratatui`, `reqwest`, `serde`, `serde_json`, and `tokio` for their amazing libraries.

For more information, please refer to the [documentation](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html).
