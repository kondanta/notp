use structopt::StructOpt;

/// List of the command line arguments
#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "basic")]
pub(crate) struct Opt {
    /// Listing existing OTP secrets.
    #[structopt(short, long)]
    pub list: Option<bool>,

    /// Adding new OTP secret
    #[structopt(short, long)]
    pub add: Option<String>,

    /// Remove existing OTP secret
    #[structopt(short, long)]
    pub delete: Option<String>,

    /// Initialize the totp file with password.
    #[structopt(short, long)]
    pub init: Option<bool>,
}

impl Opt {
    /// Parses CLI arguments
    pub(crate) fn get_cli_args() -> Self
    where
        Self: Sized,
    {
        Opt::from_args()
    }
}
