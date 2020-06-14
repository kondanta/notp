use otp::make_totp;

use super::error::{
    NotpError,
    NotpResult,
};

/// A trait that defines the functionality of a OTP to implement.
///
/// Any OTP implementation should have new to initalize the secret token and
/// generate function to get the generated 6 digit code.
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
pub(crate) struct OtpGenerator {
    secret: String,
}

impl OTP<u32> for OtpGenerator {
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
            Ok(code) => {
                if code <= 99999 {
                    Err(NotpError::Generic(format!(
                        "Something unexpected happened! Wait a minute and try \
                         it later. The code I've generated is not 6 digit. \
                         Here look: {}",
                        code
                    )))
                } else {
                    Ok(code)
                }
            }
            Err(e) => Err(NotpError::OTPStringError(e)),
        }
    }
}

#[cfg(test)]
mod tests {}
