use std::io::ErrorKind;
use std::process::Termination;
use std::process::ExitCode;
#[derive(Debug)]
pub enum Error {
    Io( std::io::Error),
}
impl  Termination  for Error{
     fn report(self) -> ExitCode{
         return ExitCode::FAILURE;
     }
}
// pub type Result<T> = std::result::Result<T, Error>;
pub fn run() -> Result<String,Error> {
    //Ok("command::run!!".to_string())
    Err(Error::Io(std::io::Error::new(ErrorKind::AddrInUse,"")))
}

