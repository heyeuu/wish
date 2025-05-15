use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::sys::link_patches;
use log::info;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // It is necessary to call this function once. 
    // Otherwise, some runtime patches implemented by esp-idf-sys might not link properly.
    link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Acquire the peripherals (pins, ADCs, etc.)
    let peripherals = Peripherals::take()?;

    // Initialize GPIO4 as an output pin
    let mut led = PinDriver::output(peripherals.pins.gpio4)?;

    // Log a message to indicate the program has started
    info!("Hello, ESP32!");

    // Infinite loop to toggle the LED
    loop {
        // Turn the LED on
        led.set_high()?;

        // Delay for 1000 milliseconds (1 second)
        FreeRtos::delay_ms(1000);

        // Turn the LED off
        led.set_low()?;

        // Delay for another 1000 milliseconds (1 second)
        FreeRtos::delay_ms(1000);
    }

    // This part is unreachable in this example as the loop never terminates.
    // However, for completeness and to handle potential future modifications:
    // Ok(())
}