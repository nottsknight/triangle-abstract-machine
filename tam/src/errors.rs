use std::fmt::{Display, Error, Formatter};

/// Represents different runtime errors.
#[derive(Debug)]
pub enum TAMError {
    /// Indicate an attempt to access memory outside the stack or heap.
    SegmentationFault(usize, usize),
    /// Indicate the stack and heap have collided.
    StackOverflow(usize),
    /// Indicate there was an attempt to divide by zero.
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
            Self::DivideByZero(loc) => {
                write!(f, "divide by zero attempted at loc {:04x}", loc)
            }
        }
    }
}

pub type TAMResult<T> = Result<T, TAMError>;
