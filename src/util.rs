use super::error::{
    NotpError,
    NotpResult,
};
use dirs::config_dir;
use rpassword::read_password_from_tty;
use std::fs::{
    create_dir_all,
    remove_dir,
};
use std::io;
use std::path::{
    Path,
    PathBuf,
};

/// Checks if the given path exists or not
pub(crate) fn does_path_exists(path: &str) -> bool {
    if let Some(p) = get_config_dir() {
        let notp_path = format!("{}/{}", p.to_str().unwrap_or_default(), path);
        return Path::new(&notp_path).exists();
    }
    false
}

fn get_config_dir() -> Option<PathBuf> {
    config_dir()
}

/// Creates folder inside the config folder.
pub(crate) fn create_folder(path: &str) -> NotpResult<()> {
    if let Some(p) = get_config_dir() {
        let notp_path = format!("{}/{}", p.to_str().unwrap_or_default(), path);
        return match create_dir_all(&notp_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(NotpError::Io(e)),
        };
    }
    Err(NotpError::Generic("Cannot find the file!".to_string()))
}

#[allow(dead_code)]
pub(crate) fn remove_folder(path: &str) -> NotpResult<()> {
    if does_path_exists(&path) {
        if let Some(p) = get_config_dir() {
            let folder_path =
                format!("{}/{}", p.to_str().unwrap_or_default(), path);
            return remove_dir(folder_path)
                .map(|_| ())
                .map_err(|e| NotpError::Generic(e.to_string()));
        }
    }
    Err(NotpError::Generic("Config path does not exist".to_string()))
}

/// Returns to the full path of the given path.
///
/// Let's say we want to get notp folder's full path. All we need to do is
/// saying `get_folder_path("notp")` and this function will return to the
/// full path such as `/home/user/.config/notp`.
pub(crate) fn get_folder_path(path: &str) -> Option<String> {
    if let Some(p) = get_config_dir() {
        let folder_path =
            format!("{}/{}", p.to_str().unwrap_or_default(), path);
        return Some(folder_path);
    }
    None
}

/// Reads text from stdin
pub(crate) fn read_from_stdin() -> NotpResult<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line)
}

/// Reads text from stdin securely
///
/// In bash, we have read -s functionality that does not print the
/// characters that typed by the user. This function also does the same.
pub(crate) fn read_from_stdin_securely() -> NotpResult<String> {
    read_password_from_tty(Some("Key: ")).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use crate::util::{
        create_folder,
        is_path_exists,
    };

    #[test]
    fn is_home_exist() {
        assert!(is_path_exists(""));
    }

    #[test]
    fn is_home_exists_should_return_false() {
        assert_eq!(false, is_path_exists("PathThatDoesNotExists"));
    }

    #[test]
    fn should_create_notp_folder() {
        create_folder("notp");
        assert_eq!(true, is_path_exists("notp"));
    }
}
