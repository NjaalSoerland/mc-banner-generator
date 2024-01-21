# Minecraft Banner Generator

This project provides a Rust-based tool for generating Minecraft banners using a genetic algorithm approach.

## Prerequisites

- Rust programming environment.
- Cargo, the Rust package manager.

## Installation

1. Clone the Repository:

   ```sh
   git clone https://github.com/NjaalSoerland/mc-banner-generator.git
   cd mc-banner-generator
   ```

2. Build the Project:
   ```sh
   cargo build --release
   ```

## Running the Program

1. To run the program, use the following command:

   ```sh
   cargo run --release
   ```

   This will execute the main functionality as implemented in `main.rs`, which includes initializing the genetic algorithm with predefined configurations and processing the image data.

2. The output, including the best banner generated, will be saved in the specified directory, typically under `./src/renders/`.

## License

This project is open-sourced under the MIT License.
