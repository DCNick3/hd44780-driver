use core::future::Future;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal_async::delay::DelayUs;

use crate::error::{Error, Result};
use crate::non_blocking::bus::DataBus;

pub struct FourBitBus<
    RS: OutputPin,
    EN: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
    D: DelayUs,
> {
    rs: RS,
    en: EN,
    d4: D4,
    d5: D5,
    d6: D6,
    d7: D7,
    delay: D,
}

impl<
        RS: OutputPin,
        EN: OutputPin,
        D4: OutputPin,
        D5: OutputPin,
        D6: OutputPin,
        D7: OutputPin,
        D: DelayUs,
    > FourBitBus<RS, EN, D4, D5, D6, D7, D>
{
    pub fn from_pins(
        rs: RS,
        en: EN,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
        delay: D,
    ) -> FourBitBus<RS, EN, D4, D5, D6, D7, D> {
        FourBitBus {
            rs,
            en,
            d4,
            d5,
            d6,
            d7,
            delay,
        }
    }

    fn write_lower_nibble(&mut self, data: u8) -> Result<()> {
        let db0: bool = (0b0000_0001 & data) != 0;
        let db1: bool = (0b0000_0010 & data) != 0;
        let db2: bool = (0b0000_0100 & data) != 0;
        let db3: bool = (0b0000_1000 & data) != 0;

        if db0 {
            self.d4.set_high().map_err(|_| Error)?;
        } else {
            self.d4.set_low().map_err(|_| Error)?;
        }

        if db1 {
            self.d5.set_high().map_err(|_| Error)?;
        } else {
            self.d5.set_low().map_err(|_| Error)?;
        }

        if db2 {
            self.d6.set_high().map_err(|_| Error)?;
        } else {
            self.d6.set_low().map_err(|_| Error)?;
        }

        if db3 {
            self.d7.set_high().map_err(|_| Error)?;
        } else {
            self.d7.set_low().map_err(|_| Error)?;
        }

        Ok(())
    }

    fn write_upper_nibble(&mut self, data: u8) -> Result<()> {
        let db4: bool = (0b0001_0000 & data) != 0;
        let db5: bool = (0b0010_0000 & data) != 0;
        let db6: bool = (0b0100_0000 & data) != 0;
        let db7: bool = (0b1000_0000 & data) != 0;

        if db4 {
            self.d4.set_high().map_err(|_| Error)?;
        } else {
            self.d4.set_low().map_err(|_| Error)?;
        }

        if db5 {
            self.d5.set_high().map_err(|_| Error)?;
        } else {
            self.d5.set_low().map_err(|_| Error)?;
        }

        if db6 {
            self.d6.set_high().map_err(|_| Error)?;
        } else {
            self.d6.set_low().map_err(|_| Error)?;
        }

        if db7 {
            self.d7.set_high().map_err(|_| Error)?;
        } else {
            self.d7.set_low().map_err(|_| Error)?;
        }
        Ok(())
    }
}

impl<
        RS: OutputPin + 'static,
        EN: OutputPin + 'static,
        D4: OutputPin + 'static,
        D5: OutputPin + 'static,
        D6: OutputPin + 'static,
        D7: OutputPin + 'static,
        D: DelayUs,
    > DataBus for FourBitBus<RS, EN, D4, D5, D6, D7, D>
{
    type WriteFuture<'a> = impl Future<Output = Result<()>> + 'a
    where
        D: 'a;

    fn write<'a>(&'a mut self, byte: u8, data: bool) -> Self::WriteFuture<'a> {
        async move {
            if data {
                self.rs.set_high().map_err(|_| Error)?;
            } else {
                self.rs.set_low().map_err(|_| Error)?;
            }
            self.write_upper_nibble(byte)?;
            // Pulse the enable pin to recieve the upper nibble
            self.en.set_high().map_err(|_| Error)?;
            self.delay.delay_ms(2).await.unwrap();
            self.en.set_low().map_err(|_| Error)?;
            self.write_lower_nibble(byte)?;
            // Pulse the enable pin to recieve the lower nibble
            self.en.set_high().map_err(|_| Error)?;
            self.delay.delay_ms(2).await.unwrap();
            self.en.set_low().map_err(|_| Error)?;
            if data {
                self.rs.set_low().map_err(|_| Error)?;
            }
            Ok(())
        }
    }
}
