use structopt::StructOpt;

/// List of the command line arguments
#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "basic")]
pub(crate) struct Opt {
    /// Listing existing OTP secrets.
    #[structopt(short, long)]
    pub list: bool,

    /// Adding new OTP secret
    #[structopt(short, long)]
    pub add: Option<String>,

    /// Remove existing OTP secret
    #[structopt(short, long)]
    #[allow(unused)]
    pub delete: Option<String>,

    /// Specific key
    #[structopt(short, long)]
    pub get: Option<String>,

    /// Encryption key for the file that totp going to use as a data source.
    #[structopt(long)]
    pub key: String,
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
