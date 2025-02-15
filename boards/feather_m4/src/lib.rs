#![no_std]
#![recursion_limit = "1024"]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use target_device::*;
use hal::prelude::*;
pub use hal::*;

use gpio::{Floating, Input, Port};
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster2, PadPin, SPIMaster1};
use hal::time::Hertz;

#[cfg(feature = "usb")]
use gpio::IntoFunction;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;
#[cfg(feature = "usb")]
use usb_device::bus::UsbBusWrapper;

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: target_device, 

    /// Analog pin 0.  Can act as a true analog output
    /// as it has a DAC (which is not currently supported
    /// by this hal) as well as input.
    pin a0 = a2,

    /// Analog Pin 1
    pin a1 = a5,
    /// Analog Pin 2
    pin a2 = b8,
    /// Analog Pin 3
    pin a3 = b9,
    /// Analog Pin 4
    pin a4 = a4,
    /// Analog Pin 5
    pin a5 = a6,

    /// Pin 0, rx
    pin d0 = b16,
    /// Pin 1, tx
    pin d1 = b17,
    /// Pin 4, PWM capable
    pin d4 = a14,

    /// Pin 5, PWM capable
    pin d5 = a16,
    /// Pin 6, PWM capable
    pin d6 = a18,
    /// Neopixel Pin
    pin neopixel = b3,
    /// Pin 9, PWM capable.  Also analog input (A7)
    pin d9 = a19,
    /// Pin 10, PWM capable
    pin d10 = a20,
    /// Pin 11, PWM capable
    pin d11 = a21,
    /// Pin 12, PWM capable
    pin d12 = a22,
    /// Pin 13, which is also attached to
    /// the red LED.  PWM capable.
    pin d13 = a23,

    /// The I2C data line
    pin sda = a12,
    /// The I2C clock line
    pin scl = a13,

    /// The SPI SCK
    pin sck = a17,
    /// The SPI MOSI
    pin mosi = b23,
    /// The SPI MISO
    pin miso = b22,

    /// The USB D- pad
    pin usb_dm = a24,
    /// The USB D+ pad
    pin usb_dp = a25,
);

/// Convenience for setting up the labelled SPI peripheral.
/// This powers up SERCOM1 and configures it for use as an
/// SPI Master in SPI Mode 0.
pub fn spi_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom1: SERCOM1,
    mclk: &mut MCLK,
    sck: gpio::Pa17<Input<Floating>>,
    mosi: gpio::Pb23<Input<Floating>>,
    miso: gpio::Pb22<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster1<
        hal::sercom::Sercom1Pad2<gpio::Pb22<gpio::PfC>>,
        hal::sercom::Sercom1Pad3<gpio::Pb23<gpio::PfC>>,
        hal::sercom::Sercom1Pad1<gpio::Pa17<gpio::PfC>>
    > {
    let gclk0 = clocks.gclk0();
    SPIMaster1::new(
        &clocks.sercom1_core(&gclk0).unwrap(),
        bus_speed.into(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom1,
        mclk,
        (miso.into_pad(port), mosi.into_pad(port), sck.into_pad(port)),
    )
}

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom2: SERCOM2,
    mclk: &mut MCLK,
    sda: gpio::Pa12<Input<Floating>>,
    scl: gpio::Pa13<Input<Floating>>,
    port: &mut Port,
) -> I2CMaster2<
        hal::sercom::Sercom2Pad0<gpio::Pa12<gpio::PfC>>,
        hal::sercom::Sercom2Pad1<gpio::Pa13<gpio::PfC>>
    > {
    let gclk0 = clocks.gclk0();
    I2CMaster2::new(
        &clocks.sercom2_core(&gclk0).unwrap(),
        bus_speed.into(),
        sercom2,
        mclk,
        sda.into_pad(port),
        scl.into_pad(port),
    )
}

#[cfg(feature = "usb")]
pub fn usb_bus(
    usb: USB,
    clocks: &mut GenericClockController,
    mclk: &mut MCLK,
    dm: gpio::Pa24<Input<Floating>>,
    dp: gpio::Pa25<Input<Floating>>,
    port: &mut Port,
) -> UsbBusWrapper<UsbBus> {
    let gclk0 = clocks.gclk0();
    dbgprint!("making usb clock");
    let usb_clock = &clocks.usb(&gclk0).unwrap();
    dbgprint!("got clock");
    UsbBusWrapper::new(UsbBus::new(
        usb_clock,
        mclk,
        dm.into_function(port),
        dp.into_function(port),
        usb,
    ))
}
