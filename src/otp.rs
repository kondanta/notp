use oath::{
    totp_raw_now,
    HashType,
};

use bytevec::{
    ByteDecodable,
    ByteEncodable,
};

use otp::make_totp;

use super::error::{
    NotpError,
    NotpResult,
};

pub(crate) trait OTP<T> {
    fn new(token: &str) -> Self;
    fn generate(
        &self,
        epoch: u64,
        time_step: u64,
    ) -> NotpResult<T>;
}

/// New implementation of otp generator.
///
/// Takes token and stores it as `String`. It does not use any complex types to
/// handle token generation.
pub(crate) struct OTPString {
    secret: String,
}

impl OTP<u32> for OTPString {
    /// Returns new OTPString instance that holds `String`: token.
    fn new(token: &str) -> Self {
        Self {
            secret: token.to_string(),
        }
    }

    /// Generates One Time Password using given otp token.
    fn generate(
        &self,
        epoch: u64,
        time_step: u64,
    ) -> NotpResult<u32> {
        let totp = make_totp(&self.secret, time_step, epoch as i64);
        match totp {
            Ok(code) => Ok(code),
            Err(e) => Err(NotpError::OTPStringError(e)),
        }
    }
}

/// OTP generator that uses `Vec<u8>`
pub(crate) struct OTPHash {
    secret: Vec<u8>,
}

impl OTP<u64> for OTPHash {
    /// Creates new OTP struct with a token.
    fn new(token: &str) -> Self {
        // Convert &str into Vector of u8
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
    /// WARNING: This function generates 5 digit codes time to times
    fn generate(
        &self,
        epoch: u64,
        time_step: u64,
    ) -> NotpResult<u64> {
        let token =
            totp_raw_now(&self.secret, 6, epoch, time_step, &HashType::SHA1);
        let len = token
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap_or_default())
            .collect::<Vec<_>>();
        if len.len() != 6 {
            return Err(NotpError::Generic(
                "OTPHash generated a number that does not even contain 6 \
                 digits."
                    .to_string(),
            ));
        }
        Ok(token)
    }
}

    }
}
