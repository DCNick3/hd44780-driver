use defmt::Format;

#[derive(Debug)]
pub struct Error;
pub type Result<T> = core::result::Result<T, Error>;

impl Format for Error {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "generic hd44780 error")
    }
}
