# B-Z Reaction Cellular Automata

## Overview

The B-Z Reaction Simulation project models the Belousov-Zhabotinsky reaction using cellular automata. This project leverages GPU compute shaders with `wgpu` to produce spiral wave patterns similar to those seen in the actual B-Z reaction. The B-Z reaction is a fascinating example of a chemical oscillator, oscillating between oxidized and reduced states, and producing beautiful patterns when done in a thin fluid layer.

<img src="https://github.com/HyperRays/CellSim/assets/107126915/42a56da3-5605-4fde-94b4-69a8c6920e6d" width=400/>
<img src="https://github.com/HyperRays/CellSim/assets/107126915/9bbc62de-a638-4493-b574-f7f4390f88b3" width=400/>

## Features

- **Cellular Automaton Model**: Simulates the abstract features of the B-Z reaction, including two end states, positive feedback, and negative feedback mechanisms.
- **GPU Acceleration**: Utilizes compute shaders to achieve high-performance simulations.
- **Graphical Interface**: Integrates with `egui` to provide an interactive graphical user interface.

## Project Structure

- `main.rs`: Entry point of the application.
- `app.rs`: Contains the main application logic.
- `compute.rs`: Manages compute operations on the GPU.
- `compute.wgsl`: WGSL shader for compute tasks related to the cellular automaton.
- `copy.wgsl`: WGSL shader for data copying operations.
- `egui.rs`: Manages the graphical user interface using `egui`.
- `renderdata.rs`: Handles rendering data and buffer management.
- `settings.rs`: Manages application settings and configurations.
- `shader.wgsl`: General purpose shader for rendering tasks.
- `state.rs`: Manages the state of the application and the cellular automaton.

## Prerequisites

- Rust (latest stable version
- `cargo` (Rust package manager, comes with the installation of Rust)

## Running the Application

To run the application, execute the following command:
    `cargo run --release`

## Usage

Upon running the application, you will be presented with an interactive graphical interface (powered by `egui`). Here, you can observe the simulated B-Z reaction and the emergence of spiral wave patterns. You can interact with various parameters and settings to see how they affect the reaction dynamics.

To change the resolution modify the GRID variable under `settings.rs`

## Contributing

Contributions are welcome! Please fork the repository and submit pull requests for any enhancements or bug fixes.

## License

This project is licensed under the Apache 2.0 License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [wgpu](https://github.com/gfx-rs/wgpu) - A cross-platform, safe, and portable WebGPU implementation.
- [egui](https://github.com/emilk/egui) - An easy-to-use immediate mode GUI in Rust.

#### Readme Made by ChatGPT
