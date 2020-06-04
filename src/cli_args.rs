use std::collections::HashMap;
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

    /// Returns to a Map that contains a pair whose key is the function and
    /// value is the invocation infromation.
    pub(crate) fn export_functions(&self) -> HashMap<OperationTypes, bool> {
        let mut m = HashMap::<OperationTypes, bool>::new();

        // Early reeturn for list. List is special. It does not take a value
        // like other operations.
        if self.list {
            let op = OperationTypes::List;
            m.insert(op, true);
            return m;
        }

        // Create `OperationTypes` by manually pairing them with variables.
        let get = self.get_operation(&self.get, "get");
        let add = self.get_operation(&self.add, "add");
        let delete = self.get_operation(&self.delete, "delete");

        m.insert(get.clone(), self.is_invoked(get));
        m.insert(add.clone(), self.is_invoked(add));
        m.insert(delete.clone(), self.is_invoked(delete));

        m
    }

    /// Checks if the function's invoked or not.
    fn is_invoked(
        &self,
        op: OperationTypes,
    ) -> bool {
        match op {
            OperationTypes::Add(ref s) => !s.is_empty(),
            OperationTypes::Get(ref s) => !s.is_empty(),
            OperationTypes::Delete(ref s) => !s.is_empty(),
            _ => false,
        }
    }

    // Finds the operation's name, and puts it into `OperationTypes` enum.
    fn get_operation<'a>(
        &self,
        variable: &Option<String>,
        type_: &'a str,
    ) -> OperationTypes {
        match type_ {
            "get" => OperationTypes::Get(variable.clone().unwrap_or_default()),
            "add" => OperationTypes::Add(variable.clone().unwrap_or_default()),
            "delete" => {
                OperationTypes::Delete(variable.clone().unwrap_or_default())
            }
            "list" => OperationTypes::List,
            _ => OperationTypes::NoType,
        }
    }
}

/// List of available operations.
///
/// `OperationTypes` is the operations that are defined in `operation.rs` file.
/// This also defines the main capabilities of `notp` application.
#[derive(Clone, Hash, Eq, PartialEq)]
pub(crate) enum OperationTypes {
    List,
    Get(String),
    Add(String),
    Delete(String),
    NoType,
}
