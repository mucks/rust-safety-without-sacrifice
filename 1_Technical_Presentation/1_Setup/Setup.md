# Setup

## Install rustup
To install Rust, we will use `rustup`, the official Rust installer and version management tool. Follow these steps:
1. Open your terminal.
2. Run the following command to download and install `rustup`:
3. ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
4. Follow the on-screen instructions to complete the installation.
5. After installation, you may need to restart your terminal or run `source $HOME/.cargo/env` to update your environment variables.
6. Verify the installation by checking the Rust version:
7. ```sh
   rustc --version
   ```
## Test if cargo works
`cargo` is Rust's package manager and build system. To test if `cargo` is working correctly, follow these steps:
1. Create a new Rust project by running:
2. ```sh
    cargo new hello_world
3. Navigate into the project directory:
4. ```sh
    cd hello_world
5. Build and run the project using:
6. ```sh
    cargo run
7. You should see the output:
8. ```sh
    Hello, world!
9. If you see this output, `cargo` is working correctly, and you are ready to start coding in Rust!

## Open the project in your favorite editor
You can use any text editor or IDE that supports Rust. Some popular choices include:
- Visual Studio Code with the Rust extension
- IntelliJ IDEA with the Rust plugin
- Sublime Text with Rust packages
- Vim or Neovim with Rust plugins
Open the `hello_world` project folder in your chosen editor to start coding.

## Turn off AI assistance
If you are using an IDE or text editor with AI assistance (like GitHub Copilot), you may want to turn it off to focus on learning Rust without distractions. Refer to your editor's documentation for instructions on how to disable AI features.