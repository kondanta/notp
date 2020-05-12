use structopt::StructOpt;

/// List of the command line arguments
#[derive(StructOpt, Debug, Clone)]
#[structopt(
    name = "NOTP",
    about = "CLI Utility that generates One Time Passwords for given services."
)]
pub(crate) struct Opt {
    /// Listing existing OTP secrets.
    #[structopt(short, long)]
    pub list: bool,

    /// Adding new OTP secret
    #[structopt(short, long)]
    pub add: Option<String>,

    /// Remove existing OTP secret
    #[structopt(short, long)]
    pub delete: Option<String>,

    /// Get OTP code for the selected secret.
    #[structopt(short, long)]
    pub get: Option<String>,

    /// Encryption key for the file that notp going to use as a data source.
    #[structopt(
        long,
        required_unless = "list",
        required_unless = "delete",
        conflicts_with = "stdin"
    )]
    pub key: Option<String>,

    /// Suppresses the OTP output and just prints the code.
    #[structopt(short, long)]
    pub quiet: bool,

    /// Reads key securely from stdin.
    ///
    /// `stdin` option does not print characters on the screen. It works like
    /// bash's read -s functionality.
    #[structopt(
        long,
        required_unless = "list",
        required_unless = "delete",
        conflicts_with = "key"
    )]
    pub stdin: bool,
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
