use structopt::StructOpt;

/// List of the command line arguments
#[derive(StructOpt, Debug, Clone)]
#[structopt(
    name = "NOTP",
    about = "CLI Utility that generates One Time Passwords for given services."
)]
pub(crate) struct Opt {
    #[structopt(subcommand)]
    pub operations: Option<OperationList>,
}

#[derive(StructOpt, Debug, Clone)]
pub(crate) enum OperationList {
    /// Adding new OTP secret
    #[structopt(name = "add")]
    Add(AddCommand),

    /// Remove existing OTP secret
    #[structopt(name = "delete")]
    Delete(DeleteCommand),

    /// Get OTP code for the selected secret.
    #[structopt(name = "get")]
    Get(GetCommand),
}

#[derive(StructOpt, Debug, Clone)]
pub(crate) struct AddCommand {
    pub name: String,

    /// Encryption key for the file that notp going to use as a data source.
    #[structopt(required_unless = "stdin", long, conflicts_with = "stdin")]
    pub key: Option<String>,
    /// Reads key securely from stdin.
    ///
    /// `stdin` option does not print characters on the screen. It works like
    /// bash's read -s functionality.

    #[structopt(required_unless = "key", long, conflicts_with = "key")]
    pub stdin: bool,
}

#[derive(StructOpt, Debug, Clone)]
pub(crate) struct DeleteCommand {
    pub name: String,
}

#[derive(StructOpt, Debug, Clone)]
pub(crate) struct GetCommand {
    pub name: String,

    /// Suppresses the OTP output and just prints the code.
    #[structopt(short, long)]
    pub quiet: bool,

    /// Encryption key for the file that notp going to use as a data source.
    #[structopt(required_unless = "stdin", long, conflicts_with = "stdin")]
    pub key: Option<String>,

    /// Reads key securely from stdin.
    ///
    /// `stdin` option does not print characters on the screen. It works like
    /// bash's read -s functionality.
    #[structopt(required_unless = "key", long, conflicts_with = "key")]
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
