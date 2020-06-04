use oath::{
    totp_raw_now,
    HashType,
};

use bytevec::{
    ByteDecodable,
    ByteEncodable,
};

/// OTP generator
pub(crate) struct OTP {
    secret: Vec<u8>,
}

impl OTP {
    /// Creates new OTP struct with a token.
    pub(crate) fn new(token: &str) -> Self {
        let dc =
            base32::decode(base32::Alphabet::RFC4648 { padding: false }, token)
                .unwrap_or_else(|| {
                    let slice = token.as_bytes();
                    let bytes = slice.encode::<u32>().unwrap_or_default();
                    <Vec<u8>>::decode::<u32>(&bytes).unwrap_or_default()
                });
        Self { secret: dc }
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
