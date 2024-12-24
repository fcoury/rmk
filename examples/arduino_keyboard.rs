//! A "Hello World" example that can be run on an Arduino Leonardo.
//!
//! # Usage
//!
//! 1. (Optional) Connect a pushbutton switch to the D2 pin of the Leonardo, and
//! connect the other pin of the switch to GND.
//!
//! 2. Connect the Leonardo to the computer with a USB cable.
//!
//! 3. Make sure [Ravedude](https://github.com/Rahix/avr-hal/tree/main/ravedude)
//! is installed. Then "run" the example to deploy it to the Arduino:
//!
//!   ```
//!   cargo run --release --example arduino_keyboard
//!   ```
//!   

#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]
#![deny(unsafe_op_in_unsafe_fn)]

mod std_stub;

use layers::Layers;
use matrix::Matrix;
use usb_keyboard::UsbKeyboard;
use keyboard_config::{MATRIX_COLS, MATRIX_ROWS};

use arduino_hal::{
    entry,
    pac::{self, PLL},
    pins,
    port::{
        mode::Output,
        Pin,
    },
    prelude::*,
    Peripherals,
};
use atmega_usbd::{keyboard_config, layers, matrix, usb_keyboard, SuspendNotifier, UsbBus};
use avr_device::{asm::sleep, interrupt};
use usb_device::{
    class_prelude::UsbBusAllocator,
    descriptor::lang_id::LangID,
    device::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
    prelude::StringDescriptors,
};
use usbd_hid::{
    descriptor::{KeyboardReport, SerializedDescriptor},
    hid_class::HIDClass,
};

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);
    let pll = dp.PLL;
    let usb = dp.USB_DEVICE;

    // Configure PLL interface
    // prescale 16MHz crystal -> 8MHz
    pll.pllcsr.write(|w| w.pindiv().set_bit());
    // 96MHz PLL output; /1.5 for 64MHz timers, /2 for 48MHz USB
    pll.pllfrq
        .write(|w| w.pdiv().mhz96().plltm().factor_15().pllusb().set_bit());

    // Enable PLL
    pll.pllcsr.modify(|_, w| w.plle().set_bit());

    // Check PLL lock
    while pll.pllcsr.read().plock().bit_is_clear() {}

    let usb_bus = unsafe {
        static mut USB_BUS: Option<UsbBusAllocator<UsbBus<PLL>>> = None;
        &*USB_BUS.insert(UsbBus::with_suspend_notifier(usb, pll))
    };

    let hid_class = HIDClass::new(usb_bus, KeyboardReport::desc(), 1);
    let strings = StringDescriptors::new(LangID::EN)
        .manufacturer("Felipe")
        .product("KeyCoury");
    // let usb_device = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x1209, 0x0001))
    let usb_device = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x16C0, 0x0486))
        .strings(&[strings])
        .unwrap()
        .build();

    let mut matrix = Matrix::new(pins);
    let mut layers = Layers::new(); // Initialize with default keymaps
    
    // Define a pin for the status LED
    // let status_led = pins.d13.into_output().downgrade();

    let usb_keyboard = UsbKeyboard::new(usb_device, hid_class); // Pass the usb allocator to your keyboard class

    unsafe {
        USB_CTX = Some(usb_keyboard);
        MATRIX_CTX = Some(matrix);
        LAYERS_CTX = Some(layers);
    }

    // Timer interrupt setup (example using Timer1)
    unsafe {
        // Configure Timer1 for regular scanning (adjust prescaler and compare value as needed)
        // Access Timer1 through the TC1 constant
        (*pac::TC1::ptr())
            .tccr1a
            .write(|w| w.wgm1().bits(0b00)); // WGM11 and WGM10 = 0
        (*pac::TC1::ptr())
            .tccr1b
            .write(|w| w.cs1().prescale_64().wgm1().bits(0b01)); // CS1=prescale_64, WGM13 and WGM12 = 0b01 (CTC mode with OCR1A as TOP)
        (*pac::TC1::ptr()).ocr1a.write(|w| w.bits(1000)); // Example: 1kHz scanning (adjust for desired rate)
        (*pac::TC1::ptr()).timsk1.write(|w| w.ocie1a().set_bit());

        interrupt::enable();
    }

    loop {
        sleep(); // Sleep until an interrupt occurs
    }
}

static mut USB_CTX: Option<UsbKeyboard<UsbBus<PLL>>> = None;
static mut MATRIX_CTX: Option<Matrix> = None;
static mut LAYERS_CTX: Option<Layers> = None;

#[interrupt(atmega32u4)]
fn USB_GEN() {
    unsafe {
        if let Some(usb_keyboard) = &mut USB_CTX {
            usb_keyboard.poll();
        }
    }
}

#[interrupt(atmega32u4)]
fn USB_COM() {
    unsafe {
        if let Some(usb_keyboard) = &mut USB_CTX {
            usb_keyboard.poll();
        }
    }
}

// Timer1 Compare A interrupt
#[interrupt(atmega32u4)]
fn TIMER1_COMPA() {
    unsafe {
        if let Some(usb_keyboard) = &mut USB_CTX {
            if let Some(matrix) = &mut MATRIX_CTX {
                if let Some(layers) = &mut LAYERS_CTX {
                    let new_state = matrix.scan();
                    let layer = layers.get_current_layer();
                    for row in 0..MATRIX_ROWS {
                        for col in 0..MATRIX_COLS {
                            if new_state[row][col] != matrix.last_state[row][col] {
                                let keycode = layers.get_keycode(layer, row, col);
                                let is_pressed = new_state[row][col];
                                usb_keyboard.handle_keypress(keycode, is_pressed);
                            }
                        }
                    }
                    matrix.last_state = new_state;
                }
            }
        }
    }
}
