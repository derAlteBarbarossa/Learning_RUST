use std::fmt;
use std::io;
//use std::num::TryFromIntError;

#[derive(Debug)]
pub struct StatsError {
    pub message: String,
}

impl fmt::Display for StatsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<&str> for StatsError {
    fn from(s: &str) -> Self {
        StatsError {
            message: s.to_string(),
        }
    }
}

impl From<io::Error> for StatsError {
    fn from(e: io::Error) -> Self {
        StatsError {
            message: e.to_string(),
        }
    } 
}

/*
impl From<e: TryFromIntError> for StatsError {
    fn from(e: TryFromIntError) -> Self {
        StatsError {
            message: e.to_string(),
        }
    }
}
*/
