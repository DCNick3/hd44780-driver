use core::future::Future;
use embassy_traits::delay::Delay;
use embassy_traits::i2c::I2c;

use crate::error::Result;
use crate::non_blocking::bus::DataBus;

pub struct I2CBus<I2C: I2c, D: Delay> {
    i2c_bus: I2C,
    address: u8,
    delay: D,
}

const BACKLIGHT: u8 = 0b0000_1000;
const ENABLE: u8 = 0b0000_0100;
// const READ_WRITE: u8 = 0b0000_0010; // Not used as no reading of the `HD44780` is done
const REGISTER_SELECT: u8 = 0b0000_0001;

impl<I2C: I2c, D: Delay> I2CBus<I2C, D> {
    pub fn new(i2c_bus: I2C, address: u8, delay: D) -> I2CBus<I2C, D> {
        I2CBus {
            i2c_bus,
            address,
            delay,
        }
    }
}

impl<I2C: I2c + 'static, D: Delay> Delay for I2CBus<I2C, D> {
    type DelayFuture<'a>
    where
        D: 'a,
    = impl Future<Output = ()> + 'a;

    /// Future that completes after now + millis
    fn delay_ms(&mut self, millis: u64) -> Self::DelayFuture<'_> {
        self.delay.delay_ms(millis)
    }

    /// Future that completes after now + micros
    fn delay_us(&mut self, micros: u64) -> Self::DelayFuture<'_> {
        self.delay.delay_us(micros)
    }
}

impl<I2C: I2c + 'static, D: Delay> DataBus for I2CBus<I2C, D> {
    type WriteFuture<'a>
    where
        D: 'a,
    = impl Future<Output = Result<()>> + 'a;

    fn write<'a>(&'a mut self, byte: u8, data: bool) -> Self::WriteFuture<'a> {
        async move {
            let rs = match data {
                false => 0u8,
                true => REGISTER_SELECT,
            };
            
            let write_chain = [
                // using the same hack as arduino lib (https://github.com/duinoWitchery/hd44780/):
                // > Cheat here by raising E at the same time as setting control lines
                // > This violates the spec but seems to work realiably.
                // also we send both nibbles in one i2c transaction (it's nice =))
                // I think using DMA we can actually offload even more work off cpu sacrificing memory usage
                // but no DMA yet + will need to change the library structure... Uncool
                rs | BACKLIGHT | (byte & 0xF0)        | ENABLE, 
                rs | BACKLIGHT | (byte & 0xF0),

                rs | BACKLIGHT | ((byte & 0x0F) << 4) | ENABLE, 
                rs | BACKLIGHT | ((byte & 0x0F) << 4),
            ];


            let _ = self
                .i2c_bus
                    .write(self.address, &write_chain)
                    .await;

            // TODO: display stopped working w/o this... Maybe we want to pack everything into one chunky transaction
            self.delay.delay_ms(1).await;

            Ok(())
        }
    }
}
