use core::future::Future;

mod eightbit;
mod fourbit;
mod i2c;

pub use self::eightbit::EightBitBus;
pub use self::fourbit::FourBitBus;
pub use self::i2c::I2CBus;

use crate::error::Result;

pub trait DataBus {
    type WriteFuture<'a>: Future<Output = Result<()>>
    where
        Self: 'a;

    fn write<'a>(&'a mut self, byte: u8, data: bool) -> Self::WriteFuture<'a>;

    // TODO
    // fn read(...)
}
