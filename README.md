# hat-rs
Hybrid Asynchronous Tasks

## Status
Work in progress, major changes in API will be made.

## Requirements
- Rust 1.70+ (Nightly);
- Heap allocation (min 1KB);
- Cortex-M architecture.

## Getting started
### Examples
To run an example, run in terminal:
```shell
cargo run --example <EXAMPLE>
```
To run blinky example on board `NUCLEO-F413ZH`, run in terminal:
```shell
cargo run --example blinky
```

### Template `[Not working yet]`
To generate a new projeto using hat-rs, is recomended to use cargo-generate. The cargo generate could be installed by:
```shell
cargo install cargo-generate
```
After installation, run the bellow command to generate a new hat-rs project:
```shell
cargo generate matheuswhite/hat-rs-template --name <YOUR_PROJECT_NAME>
```

### Cargo.toml
Add this lines to your `Cargo.toml`, plus the HAL crate of your MCU:
```toml
hat = { version = "0.1.0", git = "https://github.com/matheuswhite/hat-rs" }
rtt-target = { version = "0.3.1", features = ["cortex-m"] }
cortex-m = { version = "0.7.6", features = ["critical-section-single-core"] }
cortex-m-rt = "0.7"
embedded-alloc = "0.5.0"
critical-section = "1.1.0"
```

## How to develop [WIP]
### Cargo.toml
1. Add hat-rs crate to Cargo.toml;
2. Add hat-rs dependencies to Cargo.toml;
3. Add HAL crate to Cargo.toml.

### main.rs
1. Add `#![no_std]` and `#![no_main]` at top of file;
2. Add panic handler;
3. Import MCU HAL crate to file scope;

4. Add `use hat::prelude::*;`;
5. Create the main task `async fn main() {}`;
6. Add at the top of main task the hat main attribute: `#[hat::main(1024)]` and as argument pass the heap size.
7. Use the main task to initial configuration and to spawn other tasks;
8. Before use a delay, initialize the framework time manager, using the function `init_timer` and passing as argument the SysTick object (SYST of cortex_m Peripherals) and the system (AHB) clock (in Hz);
