// Based on the Arduino LiquidCrystal_I2C Library
// https://github.com/johnrickman/LiquidCrystal_I2C

use esp_idf_hal::i2c::*;
use esp_idf_hal::delay::{FreeRtos, BLOCK};

const LCD_ENTRYSHIFTDECREMENT: u8 = 0x00;
const LCD_DISPLAYCONTROL: u8 = 0x08;
const LCD_SETDDRAMADDR: u8 = 0x80;
const LCD_CLEARDISPLAY: u8 = 0x01;
const LCD_ENTRYMODESET: u8 = 0x04;
const LCD_DISPLAYMOVE: u8 = 0x08;
const LCD_FUNCTIONSET: u8 = 0x20;
const LCD_CURSORSHIFT: u8 = 0x10;
const LCD_NOBACKLIGHT: u8 = 0x00;
const LCD_RETURNHOME: u8 = 0x02;
const LCD_ENTRYLEFT: u8 = 0x02;
const LCD_DISPLAYON: u8 = 0x04;
const LCD_CURSOROFF: u8 = 0x00;
const LCD_MOVERIGHT: u8 = 0x04;
const LCD_BACKLIGHT: u8 = 0x08;
const LCD_MOVELEFT: u8 = 0x00;
const LCD_BLINKOFF: u8 = 0x00;
const LCD_4BITMODE: u8 = 0x00;
const LCD_5X8DOTS: u8 = 0x00;
const LCD_ADDRESS: u8 = 0x27;
const LCD_2LINE: u8 = 0x08;
const EN: u8 = 0x04;
const RS: u8 = 0x01;

static mut DISPLAY_MODE: u8 = 0;
static mut DISPLAY_CONTROL: u8 = 0;
static mut BACKLIGHT: u8 = LCD_NOBACKLIGHT;

pub fn init(i2c: &mut I2cDriver) -> anyhow::Result<()> {
    let display_function = LCD_4BITMODE | LCD_2LINE | LCD_5X8DOTS;
    FreeRtos::delay_ms(50);

    unsafe {
        let _ = expander_write(i2c, BACKLIGHT);
    }
    
    FreeRtos::delay_ms(1000);
    
    for _ in 0.. 3 {
        unsafe {
            let cmd = (0x03 << 4) | BACKLIGHT;
            let _ = write4bits(i2c, cmd);
        }
        FreeRtos::delay_us(4500);
    }

    unsafe {
        let cmd = (0x02 << 4) | BACKLIGHT;
        let _ = write4bits(i2c, cmd);
    }
    
    let cmd = LCD_FUNCTIONSET | display_function;
    let _ = send(i2c, cmd, 0x0);
    
    unsafe { 
        DISPLAY_CONTROL = LCD_DISPLAYON | LCD_CURSOROFF | LCD_BLINKOFF;
    }

    display(i2c);
    
    let _ = clear(i2c);
    
    unsafe {
        DISPLAY_MODE = LCD_ENTRYLEFT | LCD_ENTRYSHIFTDECREMENT;
        let cmd = LCD_ENTRYMODESET | DISPLAY_MODE;
        let _ = send(i2c, cmd, 0x0);
    }
    
    let _ = send(i2c, LCD_RETURNHOME, 0x0);
    FreeRtos::delay_us(2000);

    Ok(())
}

pub fn print(i2c: &mut I2cDriver, ch: char) {
    let data = ch as u8;
    let _ = send(i2c, data, RS);
}

pub fn print_str(i2c: &mut I2cDriver, str: &str) {
    for ch in str.chars(){
        let _ = print(i2c, ch);
    }
}

pub fn set_cursor(i2c: &mut I2cDriver, col: u8, row: usize) {
	let row_offsets = [ 0x00, 0x40, 0x14, 0x54 ];
    let cmd = LCD_SETDDRAMADDR | col + row_offsets[row];
	let _ = send(i2c, cmd, 0x0);
}

pub fn scroll_left(i2c: &mut I2cDriver) {
    let cmd = LCD_CURSORSHIFT | LCD_DISPLAYMOVE | LCD_MOVELEFT;
	let _ = send(i2c, cmd, 0x0);
}

pub fn scroll_right(i2c: &mut I2cDriver) {
    let cmd = LCD_CURSORSHIFT | LCD_DISPLAYMOVE | LCD_MOVERIGHT;
	let _ = send(i2c, cmd, 0x0);
}

pub fn clear(i2c: &mut I2cDriver) {
    let _ = send(i2c, LCD_CLEARDISPLAY, 0x0);
    FreeRtos::delay_us(2000);
}

pub fn backlight(i2c: &mut I2cDriver) {
    unsafe {
        BACKLIGHT = LCD_BACKLIGHT;
        let _ = expander_write(i2c, BACKLIGHT);
    }
}

pub fn no_backlight(i2c: &mut I2cDriver) {
    unsafe {
        BACKLIGHT = LCD_NOBACKLIGHT;
        let _ = expander_write(i2c, BACKLIGHT);
    }
}

pub fn display(i2c: &mut I2cDriver) {
    unsafe {
        DISPLAY_CONTROL |= LCD_DISPLAYON;
        let cmd = LCD_DISPLAYCONTROL | DISPLAY_CONTROL;
        let _ = send(i2c, cmd, 0x0);
    }
}

pub fn no_display(i2c: &mut I2cDriver) {
    unsafe {
        DISPLAY_CONTROL &= !LCD_DISPLAYON;
        let cmd = LCD_DISPLAYCONTROL | DISPLAY_CONTROL;
        let _ = send(i2c, cmd, 0x0);
    }
}

pub fn left_to_right(i2c: &mut I2cDriver) {
    unsafe {
        DISPLAY_MODE |= LCD_ENTRYLEFT;
        let cmd = LCD_ENTRYMODESET | DISPLAY_MODE;
        let _ = send(i2c, cmd, 0x0);
    }
}

pub fn right_to_left(i2c: &mut I2cDriver) {
    unsafe {
        DISPLAY_MODE &= !LCD_ENTRYLEFT;
        let cmd = LCD_ENTRYMODESET | DISPLAY_MODE;
        let _ = send(i2c, cmd, 0x0);
    }	
}

fn write4bits(i2c: &mut I2cDriver, data: u8) {
    let _ = expander_write(i2c, data);
    let _ = pulse_en(i2c, data);
}

fn expander_write(i2c: &mut I2cDriver, data: u8) -> anyhow::Result<()> {
    let bytes = [0, data];
    i2c.write(LCD_ADDRESS, &bytes, BLOCK)?;
    Ok(())
}

fn send(i2c: &mut I2cDriver, cmd: u8, mode: u8) {
    let highnib: u8 = cmd & 0xf0;
    let lownib: u8 =(cmd<<4)&0xf0;

    unsafe {
        let cmd = (highnib | mode) | BACKLIGHT;
        let _ = write4bits(i2c, cmd);

        let cmd = (lownib | mode) | BACKLIGHT;
        let _ = write4bits(i2c, cmd);
    }
}

fn pulse_en (i2c: &mut I2cDriver, cmd: u8) -> anyhow::Result<()> {
    unsafe {
        let pulse = (cmd | EN) | BACKLIGHT;
        let _ = expander_write(i2c, pulse);
    }
    FreeRtos::delay_us(1);

    unsafe {
        let pulse = (cmd & !EN) | BACKLIGHT;
        let _ = expander_write(i2c, pulse);
    }
    FreeRtos::delay_us(50);
    
    Ok(())
}