#![no_std]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::prelude::*;
pub use hal::target_device::*;
pub use hal::*;

use gpio::{Floating, Input, Port};
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster3, PadPin, SPIMaster4};
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
    pin a1 = b8,
    /// Analog Pin 2
    pin a2 = b9,
    /// Analog Pin 3
    pin a3 = a4,
    /// Analog Pin 4
    pin a4 = a5,
    /// Analog Pin 5
    pin a5 = b2,

    /// Pin 0, rx
    pin d0 = a11,
    /// Pin 1, tx
    pin d1 = a10,
    /// Pin 5, PWM capable
    pin d5 = a15,
    /// Pin 6, PWM capable
    pin d6 = a20,
    /// Pin 9, PWM capable.  Also analog input (A7)
    pin d9 = a7,
    /// Pin 10, PWM capable
    pin d10 = a18,
    /// Pin 11, PWM capable
    pin d11 = a16,
    /// Pin 12, PWM capable
    pin d12 = a19,
    /// Pin 13, which is also attached to
    /// the red LED.  PWM capable.
    pin d13 = a17,

    /// The I2C data line
    pin sda = a22,
    /// The I2C clock line
    pin scl = a23,

    /// The SPI SCK
    pin sck = b11,
    /// The SPI MOSI
    pin mosi = b10,
    /// The SPI MISO
    pin miso = a12,

    /// The USB D- pad
    pin usb_dm = a24,
    /// The USB D+ pad
    pin usb_dp = a25,
);

/// Convenience for setting up the labelled SPI peripheral.
/// This powers up SERCOM4 and configures it for use as an
/// SPI Master in SPI Mode 0.
pub fn spi_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom4: SERCOM4,
    pm: &mut PM,
    sck: gpio::Pb11<Input<Floating>>,
    mosi: gpio::Pb10<Input<Floating>>,
    miso: gpio::Pa12<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster4<
        hal::sercom::Sercom4Pad0<gpio::Pa12<gpio::PfD>>,
        hal::sercom::Sercom4Pad2<gpio::Pb10<gpio::PfD>>,
        hal::sercom::Sercom4Pad3<gpio::Pb11<gpio::PfD>>
    > {
    let gclk0 = clocks.gclk0();
    SPIMaster4::new(
        &clocks.sercom4_core(&gclk0).unwrap(),
        bus_speed.into(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom4,
        pm,
        (miso.into_pad(port), mosi.into_pad(port), sck.into_pad(port)),
    )
}

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom3: SERCOM3,
    pm: &mut PM,
    sda: gpio::Pa22<Input<Floating>>,
    scl: gpio::Pa23<Input<Floating>>,
    port: &mut Port,
) -> I2CMaster3<
        hal::sercom::Sercom3Pad0<hal::gpio::Pa22<hal::gpio::PfC>>,
        hal::sercom::Sercom3Pad1<hal::gpio::Pa23<hal::gpio::PfC>>
    > {
    let gclk0 = clocks.gclk0();
    I2CMaster3::new(
        &clocks.sercom3_core(&gclk0).unwrap(),
        bus_speed.into(),
        sercom3,
        pm,
        sda.into_pad(port),
        scl.into_pad(port),
    )
}

#[cfg(feature = "usb")]
pub fn usb_bus(
    usb: USB,
    clocks: &mut GenericClockController,
    pm: &mut PM,
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
        pm,
        dm.into_function(port),
        dp.into_function(port),
        usb,
    ))
}
