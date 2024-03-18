use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub enum TAMError {
    SegmentationFault(usize, usize),
    StackOverflow(usize),
    DivideByZero(usize),
}

impl Display for TAMError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::SegmentationFault(loc, addr) => write!(
                f,
                "access violation at loc {:04x}: {:04x} is out of bounds",
                loc, addr
            ),
            Self::StackOverflow(loc) => write!(f, "stack overflow at loc {:04x}", loc),
            Self::DivideByZero(loc) => write!(f, "divide by zero attempted at loc {:04x}", loc),
        }
    }
}

pub type TAMResult<T> = Result<T, TAMError>;
