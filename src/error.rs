use std::fmt;

pub struct BirdError;

impl fmt::Display for BirdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occured")
    }
}
