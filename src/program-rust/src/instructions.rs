use solana_program::program_error::ProgramError;
use std::convert::TryInto;

// Define a set of instructions as an enum
#[derive(Debug)]
pub enum HelloInstructions {
    Increment,
    Decrement,
    Set(u32),
}

impl HelloInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        match tag {
            0 => return Ok(HelloInstructions::Increment),
            1 => return Ok(HelloInstructions::Decrement),
            2 => {
                // ensure you are parsing an array of 4-bytes. If not, return an error
                if rest.len() != 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let val: Result<[u8; 4], _> = rest[..4].try_into();
                match val {
                    Ok(i) => return Ok(HelloInstructions::Set(u32::from_le_bytes(i))),
                    // catchall: if tag /∈ {0,1,2}, then return an error
                    _ => return Err(ProgramError::InvalidInstructionData),
                }
            }
            // catchall: if you can't match tag, then return an error
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
