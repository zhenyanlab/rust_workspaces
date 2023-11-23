use std::io::ErrorKind;
use std::process::Termination;
use std::process::ExitCode;
use add_two::Iadd;
use add_two::Iadd2;
use add_two::Isub;
use add_two::IMath;
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

pub struct Execor{
    pub j:i32
}
impl  Iadd for Execor{
    fn adds(&self,mut i: i32) -> i32{
        i=i+self.j+1;
        i
    }

}

impl  Iadd2 for Execor{
    fn adds2(&self,mut i: i32) -> i32{
        i=i+self.j+2;
        i
   }
}




impl  Isub for Execor{

    fn subs(&self,mut i: i32) -> i32{
        i=self.j+i-1;
        i
    }

}
impl IMath for Execor{

}
