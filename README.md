# Overview

This project is an LED blink Embedded Rust project targeting the STM32F410RB ARM Cortex-M4 microcontroller on an STM32 Nucleo development board.

The firmware demonstrates:

* Embedded Rust development (`no_std`)
* Cortex-M runtime setup
* GPIO configuration using `stm32f4xx-hal`
* Push-pull LED output control
* Pull-up button input handling
* Polling-based embedded firmware architecture
* Software delay loops using assembly `nop`
* RTT logging with `defmt`
* probe-rs flashing and debugging

## Project Bootstrap

This project was initially generated using the STM32 Rust template:

```bash
cargo generate --git https://github.com/burrbull/stm32-template/
```
---

# Hardware

## Board

Target board:

* STM32 Nucleo-F410RB

## Microcontroller

* STM32F410RBTx
* ARM Cortex-M4
* ARMv7E-M architecture
* Hardware floating-point support

---

# Memory Layout

Defined in `memory.x`.

| Region | Address      | Size   |
| ------ | ------------ | ------ |
| FLASH  | `0x08000000` | 128 KB |
| RAM    | `0x20000000` | 32 KB  |

---

# GPIO Configuration

| Peripheral | Pin  | Configuration    |
| ---------- | ---- | ---------------- |
| User LED   | PA5  | Push-pull output |
| Button     | PC13 | Pull-up input    |

## Button Logic

The button uses active-low logic.

* Released → HIGH
* Pressed → LOW

When the button is pressed:

* LED blinks faster
* RTT log messages indicate button press state

When released:

* LED blinks slower

---

# Firmware Architecture

## Runtime Environment

This project uses:

```rust
#![no_std]
#![no_main]
```
---

## Execution Model

The firmware uses a simple polling-based infinite loop:

```rust
loop {
    // Read button
    // Toggle LED
    // Delay
}
```

Current architecture characteristics:

* Single-threaded
* Cooperative execution
* Busy-wait software delays

---

# Software Stack

## Rust Toolchain

Target:

```text
thumbv7em-none-eabihf
```
---

# Dependencies

| Crate           | Purpose                            |
| --------------- | ---------------------------------- |
| `cortex-m`      | ARM Cortex-M low-level support     |
| `cortex-m-rt`   | Cortex-M runtime and startup       |
| `stm32f4xx-hal` | STM32F4 Hardware Abstraction Layer |
| `panic-probe`   | Panic reporting                    |
| `defmt`         | Lightweight embedded logging       |
| `defmt-rtt`     | RTT transport backend              |
| `probe-rs`      | Flashing and debugging             |

---

# Logging System

Logging is implemented using:

* `defmt`
* `defmt-rtt`

Logs are transmitted over the SWD debug interface using RTT (Real-Time Transfer).

Example log output:

```text
BUTTON: PRESSED
LED: FAST BLINK
```
---

# Panic Handling

The project uses:

```rust
panic-probe
```

Panics are reported through RTT logging.

---

# Build Configuration

Defined in:

```text
.cargo/config.toml
```

## Runner

```toml
runner = "probe-rs run --chip STM32F410RBTx"
```

## Linker

The project uses the LLVM LLD linker shipped with Rust.

## Defmt Configuration

```toml
DEFMT_LOG = "info"
```

---

# Project Structure

```text
.
├── .cargo/
│   └── config.toml
├── memory.x
├── Embed.toml
├── Cargo.toml
├── src/
│   └── main.rs
├── .vscode/
│   └── launch.json
└── target/
```

---

# Build Instructions

## Install Rust Target

```bash
rustup target add thumbv7em-none-eabihf
```

---

## Install probe-rs

```bash
cargo install probe-rs-tools
```

---

## Build Firmware

Debug build:

```bash
cargo build
```

Release build:

```bash
cargo build --release
```
---

# Flash Firmware

Using Cargo runner:

```bash
cargo run
```

Using probe-rs manually:

```bash
probe-rs run --chip STM32F410RBTx
```

---

# VSCode Debugging

The project includes a `launch.json` configuration for probe-rs debugging.

## Features

* SWD debugging
* ELF flashing
* RTT logging
* SVD peripheral visualization
* Breakpoints
* Register inspection

## Required Extensions

Recommended VSCode extensions:

* Rust Analyzer
* Debugger for probe-rs
* Even Better TOML

---

# probe-rs Configuration

Defined in:

```text
Embed.toml
```

Current configuration:

| Setting    | Value    |
| ---------- | -------- |
| Protocol   | SWD      |
| Flashing   | Enabled  |
| Chip Erase | Disabled |
| RTT UI     | Disabled |
| GDB Server | Disabled |

---

# LED Blink Behavior

## Button Released

* LED toggles slowly
* Long software delay
* RTT logs:

  * `BUTTON: RELEASED`
  * `LED: SLOW BLINK`

## Button Pressed

* LED toggles faster
* Short software delay
* RTT logs:

  * `BUTTON: PRESSED`
  * `LED: FAST BLINK`

---

# Embedded Concepts Demonstrated

This project demonstrates several foundational embedded systems concepts:

* Peripheral ownership
* Memory-mapped hardware access
* RCC clock configuration
* GPIO splitting
* Push-pull outputs
* Pull-up inputs
* Active-low buttons
* Busy-wait delays
* ARM Cortex-M startup flow
* Embedded logging
* SWD debugging
---

# Learning Goals

This project is part of a broader exploration of:

* Embedded Rust
* ARM Cortex-M architecture
* STM32 microcontrollers
* Low-level firmware design
* Debugging embedded systems
* Hardware abstraction layers
* Real-time embedded programming

---
# References

## STM32 Documentation

* STM32F410 Reference Manual
* STM32F410 Datasheet
* STM32 Nucleo User Manual
---

# License

MIT License.
