use std::fmt::{Display, Write};

pub mod error;
pub use self::error::{
    Result,
    Error
};

enum State {
    Normal,
    LeftBrace,
    RightBrace,
}
use self::State::*;

/// Format a string.
/// The format syntax is similar to the one used by `std::fmt`,
/// but very limited at the moment.
/// 
/// # Example
///
/// ```
/// use monster::incubation::format;
///
/// let fmt = "You see {{{}}} tiny {}";
/// let result = format(fmt , &[&10, &"monsters"]);
///
/// assert_eq!(result.unwrap(), "You see {10} tiny monsters");
/// ```
pub fn format(fmt: &str, args: &[&Display]) -> Result<String> {
    let mut buffer = String::with_capacity(fmt.len());

    try!(write_format(&mut buffer, fmt, args));

    Ok(buffer)
}

/// Same as `format` but writes to a generic buffer instead.
pub fn write_format<W: Write>(buffer: &mut W, fmt: &str, args: &[&Display]) -> Result<()> {
    let mut args = args.iter();
    let mut state = Normal;

    for ch in fmt.chars() {
        match state {
            Normal => match ch {
                '{' => state = LeftBrace,
                '}' => state = RightBrace,
                _   => try!(buffer.write_char(ch))
            },
            LeftBrace => match ch {
                // An escaped '{'
                '{' => {
                    try!(buffer.write_char(ch));
                    state = Normal
                },
                // An escaped '}'
                '}' => {
                    match args.next() {
                        Some(arg) => try!(write!(buffer, "{}", arg)),
                        None => return Err(Error::NotEnoughArgs)
                    };
                    state = Normal
                },
                _  => return Err(Error::UnexpectedChar) // No named placeholders allowed
            },
            RightBrace => match ch {
                '}' => {
                    try!(buffer.write_char(ch));
                    state = Normal
                },
                _ => return return Err(Error::UnexpectedRightBrace) // No standalone right brace allowed
            }
        }
    }
    
    Ok(())
}