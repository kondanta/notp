use oath::{
    totp_raw_now,
    HashType,
};

/// TOTP generator
pub(crate) struct OTP {
    secret: Vec<u8>,
}

impl OTP {
    /// Creates new OTP struct with a token.
    pub(crate) fn new(token: &str) -> Self {
        Self {
            secret: base32::decode(
                base32::Alphabet::RFC4648 { padding: false },
                token,
            )
            .expect("Cannot decode the secret!"),
        }
    }

    /// Generates 6 digit one time password.
    pub(crate) fn generate_otp(
        &self,
        digits: u32,
        epoch: u64,
        time_step: u64,
    ) -> u64 {
        totp_raw_now(&self.secret, digits, epoch, time_step, &HashType::SHA1)
    }
}
