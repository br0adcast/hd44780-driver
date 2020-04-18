use core::convert::Infallible;

use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::digital::v2::OutputPin;

use bus::DataBus;

pub struct EightBitBus<
    RS: OutputPin<Error = Infallible>,
    EN: OutputPin<Error = Infallible>,
    D0: OutputPin<Error = Infallible>,
    D1: OutputPin<Error = Infallible>,
    D2: OutputPin<Error = Infallible>,
    D3: OutputPin<Error = Infallible>,
    D4: OutputPin<Error = Infallible>,
    D5: OutputPin<Error = Infallible>,
    D6: OutputPin<Error = Infallible>,
    D7: OutputPin<Error = Infallible>,
> {
    rs: RS,
    en: EN,
    d0: D0,
    d1: D1,
    d2: D2,
    d3: D3,
    d4: D4,
    d5: D5,
    d6: D6,
    d7: D7,
}

impl<
        RS: OutputPin<Error = Infallible>,
        EN: OutputPin<Error = Infallible>,
        D0: OutputPin<Error = Infallible>,
        D1: OutputPin<Error = Infallible>,
        D2: OutputPin<Error = Infallible>,
        D3: OutputPin<Error = Infallible>,
        D4: OutputPin<Error = Infallible>,
        D5: OutputPin<Error = Infallible>,
        D6: OutputPin<Error = Infallible>,
        D7: OutputPin<Error = Infallible>,
    > EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
    pub fn from_pins(
        rs: RS,
        en: EN,
        d0: D0,
        d1: D1,
        d2: D2,
        d3: D3,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
    ) -> EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7> {
        EightBitBus {
            rs,
            en,
            d0,
            d1,
            d2,
            d3,
            d4,
            d5,
            d6,
            d7,
        }
    }

    fn set_bus_bits(&mut self, data: u8) {
        let db0: bool = (0b0000_0001 & data) != 0;
        let db1: bool = (0b0000_0010 & data) != 0;
        let db2: bool = (0b0000_0100 & data) != 0;
        let db3: bool = (0b0000_1000 & data) != 0;
        let db4: bool = (0b0001_0000 & data) != 0;
        let db5: bool = (0b0010_0000 & data) != 0;
        let db6: bool = (0b0100_0000 & data) != 0;
        let db7: bool = (0b1000_0000 & data) != 0;

        if db0 {
            self.d0.set_high().unwrap();
        } else {
            self.d0.set_low().unwrap();
        }

        if db1 {
            self.d1.set_high().unwrap();
        } else {
            self.d1.set_low().unwrap();
        }

        if db2 {
            self.d2.set_high().unwrap();
        } else {
            self.d2.set_low().unwrap();
        }

        if db3 {
            self.d3.set_high().unwrap();
        } else {
            self.d3.set_low().unwrap();
        }

        if db4 {
            self.d4.set_high().unwrap();
        } else {
            self.d4.set_low().unwrap();
        }

        if db5 {
            self.d5.set_high().unwrap();
        } else {
            self.d5.set_low().unwrap();
        }

        if db6 {
            self.d6.set_high().unwrap();
        } else {
            self.d6.set_low().unwrap();
        }

        if db7 {
            self.d7.set_high().unwrap();
        } else {
            self.d7.set_low().unwrap();
        }
    }
}

impl<
        RS: OutputPin<Error = Infallible>,
        EN: OutputPin<Error = Infallible>,
        D0: OutputPin<Error = Infallible>,
        D1: OutputPin<Error = Infallible>,
        D2: OutputPin<Error = Infallible>,
        D3: OutputPin<Error = Infallible>,
        D4: OutputPin<Error = Infallible>,
        D5: OutputPin<Error = Infallible>,
        D6: OutputPin<Error = Infallible>,
        D7: OutputPin<Error = Infallible>,
    > DataBus for EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
    fn write<D: DelayUs<u16> + DelayMs<u8>>(&mut self, byte: u8, data: bool, delay: &mut D) {
        if data {
            self.rs.set_high().unwrap();
        } else {
            self.rs.set_low().unwrap();
        }

        self.set_bus_bits(byte);

        self.en.set_high().unwrap();
        delay.delay_ms(2u8);
        self.en.set_low().unwrap();

        if data {
            self.rs.set_low().unwrap();
        }
    }
}
