use chain_core::property;

use std::{error, fmt, num::ParseIntError, str};

/// Non unique identifier of the transaction position in the
/// blockchain. There may be many transactions related to the same
/// `SlotId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockDate {
    pub epoch: Epoch,
    pub slot_id: SlotId,
}

pub type Epoch = u32;
pub type SlotId = u32;

pub const EPOCH_DURATION: SlotId = 100; // FIXME: remove, make configurable

impl BlockDate {
    pub fn first() -> BlockDate {
        BlockDate {
            epoch: 0,
            slot_id: 0,
        }
    }

    /// Get the slot following this one.
    pub fn next(&self) -> BlockDate {
        assert!(self.slot_id < EPOCH_DURATION);
        if self.slot_id + 1 == EPOCH_DURATION {
            BlockDate {
                epoch: self.epoch + 1,
                slot_id: 0,
            }
        } else {
            BlockDate {
                epoch: self.epoch,
                slot_id: self.slot_id + 1,
            }
        }
    }

    pub fn next_epoch(&self) -> BlockDate {
        BlockDate {
            epoch: self.epoch + 1,
            slot_id: 0,
        }
    }
}

impl property::BlockDate for BlockDate {
    fn from_epoch_slot_id(epoch: Epoch, slot_id: SlotId) -> Self {
        assert!(slot_id < EPOCH_DURATION);
        BlockDate {
            epoch: epoch,
            slot_id: slot_id,
        }
    }
}

impl fmt::Display for BlockDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.epoch, self.slot_id)
    }
}

// FIXME: Remove this since we decided epoch durations are not in fact
// constant.
impl From<&BlockDate> for u64 {
    fn from(date: &BlockDate) -> u64 {
        (date.epoch as u64)
            .checked_mul(EPOCH_DURATION as u64)
            .unwrap()
            .checked_add(date.slot_id as u64)
            .unwrap()
    }
}

impl std::ops::Sub for BlockDate {
    type Output = u64;
    fn sub(self, other: BlockDate) -> u64 {
        u64::from(&self).checked_sub(u64::from(&other)).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockDateParseError {
    DotMissing,
    BadEpochId(ParseIntError),
    BadSlotId(ParseIntError),
}

const EXPECT_FORMAT_MESSAGE: &'static str = "expected block date format EPOCH.SLOT";

impl fmt::Display for BlockDateParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BlockDateParseError::*;
        match self {
            DotMissing => write!(f, "{}", EXPECT_FORMAT_MESSAGE),
            BadEpochId(_) => write!(f, "invalid epoch ID, {}", EXPECT_FORMAT_MESSAGE),
            BadSlotId(_) => write!(f, "invalid slot ID, {}", EXPECT_FORMAT_MESSAGE),
        }
    }
}

impl error::Error for BlockDateParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use BlockDateParseError::*;
        match self {
            DotMissing => None,
            BadEpochId(e) => Some(e),
            BadSlotId(e) => Some(e),
        }
    }
}

impl str::FromStr for BlockDate {
    type Err = BlockDateParseError;

    fn from_str(s: &str) -> Result<BlockDate, BlockDateParseError> {
        let (ep, sp) = match s.find('.') {
            None => return Err(BlockDateParseError::DotMissing),
            Some(pos) => (&s[..pos], &s[(pos + 1)..]),
        };
        let epoch = str::parse::<Epoch>(ep).map_err(BlockDateParseError::BadEpochId)?;
        let slot_id = str::parse::<SlotId>(sp).map_err(BlockDateParseError::BadSlotId)?;
        Ok(BlockDate { epoch, slot_id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn parse_no_dot() {
        let err = "42".parse::<BlockDate>().unwrap_err();
        assert_eq!(err, BlockDateParseError::DotMissing);
    }

    #[test]
    fn parse_epoch_slot_id() {
        let date = "42.12".parse::<BlockDate>().unwrap();
        assert_eq!(
            date,
            BlockDate {
                epoch: 42,
                slot_id: 12
            }
        );
    }

    #[test]
    fn parse_bad_epoch() {
        let err = "BAD.12".parse::<BlockDate>().unwrap_err();
        if let BlockDateParseError::BadEpochId(_) = err {
            println!("{}: {}", err, err.source().unwrap());
        } else {
            panic!("unexpected error {:?}", err);
        }
    }

    #[test]
    fn parse_bad_slotid() {
        let err = "42.BAD".parse::<BlockDate>().unwrap_err();
        if let BlockDateParseError::BadSlotId(_) = err {
            println!("{}: {}", err, err.source().unwrap());
        } else {
            panic!("unexpected error {:?}", err);
        }
    }
}
