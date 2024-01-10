use colored::Colorize;
use crate::consts::ERROR_RGB;

pub struct Error;

pub enum ErrorType {
    Syntax,
    Typerr,
    OS,
}

impl ToString for ErrorType {
    fn to_string(&self) -> String {
        match self {
            ErrorType::OS => "OSError",
            ErrorType::Syntax => "SyntaxError",
            ErrorType::Typerr => "TypeError",
        }.to_string()
    }
}

impl Error {
   pub fn throw(_type: ErrorType, err: &str) {
      let t_string = _type.to_string();
      let (r, g, b) = ERROR_RGB;
      let error = err.truecolor(r, g, b);

      println!("{}: {}", t_string, error)
   }
}
