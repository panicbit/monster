use std::fmt::Display;

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
pub fn format(fmt: &str, args: &[&Display]) -> Option<String> {
    use std::fmt::Write;
    let mut args = args.iter();
    let mut result = String::with_capacity(fmt.len());
    let mut state = Normal;

    for ch in fmt.chars() {
        match state {
            Normal => match ch {
                '{' => state = LeftBrace,
                '}' => state = RightBrace,
                _   => result.push(ch)
            },
            LeftBrace => match ch {
                // An escaped '{'
                '{' => {
                    result.push(ch);
                    state = Normal
                },
                // An escaped '}'
                '}' => {
                    match args.next() {
                        Some(arg) => write!(result, "{}", arg),
                        None => return None
                    };
                    state = Normal
                },
                _  => return None // No named placeholders allowed
            },
            RightBrace => match ch {
                '}' => {
                    result.push(ch);
                    state = Normal
                },
                _ => return None // No standalone right brace allowed
            }
        }
    }
    
    Some(result)
}
