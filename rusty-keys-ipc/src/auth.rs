/// Authenticate the IPC connection between the daemon and the client.

use std::{collections::HashMap, num::NonZeroU32};
use aws_lc_rs::{digest, pbkdf2};

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA384;
const CREDENTIAL_LEN: usize = digest::SHA384_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

struct PasswordIPC {
    pbkdf2_iterations: NonZeroU32, 
}

impl PasswordIPC {
    
} 