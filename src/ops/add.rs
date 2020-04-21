use crate::file::{
    read_secret,
    Crypt,
};

use std::io::{
    stdout,
    Error,
    Write,
};

/// Adds new secret into data source
pub(crate) fn add(
    name: String,
    secret: String,
) -> Result<(), Error> {
    let c = Crypt::new(secret);
    print!("Please enter the secret: ");
    // we need to flush stdout in order to fetch correct input from stdin.
    stdout().flush()?;
    c.write_into_file(name)?;
    Ok(())
}
