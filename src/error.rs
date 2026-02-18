use derive_more::From;
use embedded_io::ErrorKind;

#[derive(Debug, From)]
pub enum Error {
    WarmUp,
    Conversion,
    Write(ErrorKind),
}
