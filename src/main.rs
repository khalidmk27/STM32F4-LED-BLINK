#![deny(unsafe_code)] // Completely forbids unsafe Rust in this file
#![no_main] // We are not using the normal Rust main()
#![no_std] // No standard library because MCU has no OS

// Panic handler.
// If program crashes, panic information is sent through defmt/RTT.
use panic_probe as _;

// RTT transport layer for defmt logging.
// This sends logs over the SWD debug connection.
use defmt_rtt as _;

// Entry macro for Cortex-M bare metal programs.
use cortex_m_rt::entry;

// HAL imports:
// pac      -> Peripheral Access Crate (direct hardware registers)
// prelude  -> common helper traits/functions
use stm32f4xx_hal::{pac, prelude::*};

// #[entry] tells cortex-m-rt:
// "Start execution from this function after reset"
#[entry]
fn main() -> ! {
    // Take ownership of STM32 peripherals.
    // This gives access to RCC, GPIO, TIMERS, USART, etc.
    let dp = pac::Peripherals::take().unwrap();

    // RCC = Reset and Clock Control peripheral.
    // constrain() converts raw peripheral into safe HAL abstraction.
    let mut rcc = dp.RCC.constrain();

    // Split GPIOA into independent pins.
    // Without split(), GPIOA is one large peripheral block.
    let gpioa = dp.GPIOA.split(&mut rcc);

    // Split GPIOC into independent pins.
    let gpioc = dp.GPIOC.split(&mut rcc);

    // Configure PA5 as push-pull output.
    // PA5 is connected to onboard LED LD2.
    //
    // Push-pull means:
    // - MCU can actively drive HIGH
    // - MCU can actively drive LOW
    let mut led = gpioa.pa5.into_push_pull_output();

    // Configure PC13 as input with internal pull-up resistor.
    //
    // Pull-up resistor keeps pin HIGH normally.
    // When button is pressed:
    // PC13 gets connected to GND -> LOW
    let button = gpioc.pc13.into_pull_up_input();

    // Infinite loop.
    loop {
        // Read button state.
        //
        // is_low() means:
        // voltage on PC13 is LOW
        //
        // Since button connects PC13 to GND when pressed:
        // LOW = button pressed
        if button.is_low() {
            // Toggle LED state.
            //
            // If LED was ON -> OFF
            // If LED was OFF -> ON
            led.toggle();

            // Send log message through RTT.
            //
            // You can see this in:
            // cargo run
            //
            // or probe-rs terminal output.
            defmt::info!("BUTTON: PRESSED");

            // Another log message.
            defmt::info!("LED: FAST BLINK");

            // Software delay loop.
            //
            // nop() = "No Operation"
            //
            // CPU does nothing for one instruction cycle.
            //
            // Smaller delay:
            // LED blinks faster
            for _ in 0..50_000 {
                cortex_m::asm::nop(); //No Operation
            }
        } else {
            // Button not pressed path.

            // Toggle LED again.
            led.toggle();

            // Log button state.
            defmt::info!("BUTTON: RELEASED");

            // Log LED mode.
            defmt::info!("LED: SLOW BLINK");

            // Bigger delay:
            // LED blinks slower
            for _ in 0..300_000 {
                cortex_m::asm::nop(); //No Operation
            }
        }
    }
}
