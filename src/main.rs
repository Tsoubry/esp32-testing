use esp_idf_hal::i2c::*;
use esp_idf_hal::prelude::*;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::peripherals::Peripherals;

mod lcd;

fn main() -> anyhow::Result<()> {
    let peripherals = Peripherals::take().unwrap();
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio6;
    let scl = peripherals.pins.gpio5;

    let config = I2cConfig::new().baudrate(100.kHz().into());
    let mut i2c = I2cDriver::new(i2c, sda, scl, &config)?;

    let _ = lcd::init(&mut i2c);
    let _ = lcd::backlight(&mut i2c);

    let _ = lcd::set_cursor(&mut i2c, 0, 1);
    let _ = lcd::print_str(&mut i2c, "    Hello");
    let _ = lcd::set_cursor(&mut i2c, 0, 2);
    let _ = lcd::print_str(&mut i2c, "   World!");

    loop {
        for _ in 0..3 {
            lcd::scroll_left(&mut i2c);
            FreeRtos::delay_ms(100);
        }

        for _ in 0..2 {
            for _ in 0..6 {
                lcd::scroll_right(&mut i2c);
                FreeRtos::delay_ms(100);
            }

            for _ in 0..6 {
                lcd::scroll_left(&mut i2c);
                FreeRtos::delay_ms(100);
            }
        }

        for _ in 0..3 {
            lcd::scroll_right(&mut i2c);
            FreeRtos::delay_ms(100);
        }

        for _ in 0..3 {
            lcd::no_backlight(&mut i2c);
            FreeRtos::delay_ms(250);
            lcd::backlight(&mut i2c);
            FreeRtos::delay_ms(250);
        }
    }
}