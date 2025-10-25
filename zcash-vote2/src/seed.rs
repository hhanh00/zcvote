use rand_core::{OsRng, RngCore};

use crate::{IntoAnyhow, VoteResult};

pub fn generate_seed() -> VoteResult<String> {
    let mut entropy = [0u8; 32];
    OsRng.fill_bytes(&mut entropy);
    let m = bip39::Mnemonic::from_entropy(&entropy).anyhow()?;
    Ok(m.to_string())
}
