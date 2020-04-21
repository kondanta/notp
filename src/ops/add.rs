use crate::file::Crypt;
use std::io::{
    stdout,
    Error,
    Write,
};

/// Adds new secret into data source
pub(crate) fn add(
    name: String,
    key: &str,
) -> Result<(), Error> {
    let c = Crypt::new(key);
    print!("Please enter the secret: ");
    // we need to flush stdout in order to fetch correct input from stdin.
    stdout().flush()?;
    c.write_into_file(name)?;
    Ok(())
}
