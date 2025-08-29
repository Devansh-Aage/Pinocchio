use pinocchio::program_error::ProgramError;

#[derive(Clone, PartialEq)]
pub enum EscrowError {
    //account not a signer
    NotSigner,
    // overflow error
    WriteOverflow,
    // invalid instruction data
    InvalidInstructionData,
    // invalid Account data
    InvalidAccountData,
    // pda mismatch
    PdaMismatch,
    // Invalid Owner
    InvalidOwner,
    // Invalid Address
    InvalidAddress,
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        Self::Custom(e as u32)
    }
}
