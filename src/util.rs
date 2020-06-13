use super::error::{
    NotpError,
    NotpResult,
};
use dirs::config_dir;
use rpassword::read_password_from_tty;
use std::fs::{
    create_dir_all,
    remove_dir_all,
};
use std::io;
use std::path::{
    Path,
    PathBuf,
};

/// Checks if the given path exists or not
pub(crate) fn does_path_exists(path: &str) -> bool {
    get_config_dir()
        .map_or_else(
            || Err(NotpError::Generic("Path does not exists.".to_string())),
            |path_buf| {
                let notp_path = format!(
                    "{}/{}",
                    path_buf.to_str().unwrap_or_default(),
                    path
                );
                Ok(Path::new(&notp_path).exists())
            },
        )
        .unwrap_or_else(|_| false)
}

fn get_config_dir() -> Option<PathBuf> {
    config_dir()
}

/// Creates folder inside the config folder.
pub(crate) fn create_folder(path: &str) -> NotpResult<()> {
    if path.is_empty() {
        Err(NotpError::Generic(
            "Path is empty! Provide a path!".to_string(),
        ))
    } else {
        get_config_dir().map_or_else(
            || Err(NotpError::Generic("Cannot get PathBuf.".to_string())),
            |path_buf| {
                let notp_path = format!(
                    "{}/{}",
                    path_buf.to_str().unwrap_or_default(),
                    path
                );
                create_dir_all(&notp_path).map_err(|e| NotpError::Io(e))
            },
        )
    }
}

/// Takes a folder path and removes the folder's content and folder itself. And
/// returns to `NotpResult`.
#[allow(dead_code)]
pub(crate) fn remove_folder(path: &str) -> NotpResult<()> {
    if does_path_exists(&path) {
        get_config_dir().map_or_else(
            || Err(NotpError::Generic("Cannot get PathBuf.".to_string())),
            |path_buf| {
                let folder_path = format!(
                    "{}/{}",
                    path_buf.to_str().unwrap_or_default(),
                    path
                );
                remove_dir_all(folder_path).map_err(|e| NotpError::Io(e))
            },
        )
    } else {
        Err(NotpError::Generic("Cannot Find the folder!".to_string()))
    }
}

/// Returns to the full path of the given path.
///
/// Let's say we want to get notp folder's full path. All we need to do is
/// saying `get_folder_path("notp")` and this function will return to the
/// full path such as `/home/user/.config/notp`.
pub(crate) fn get_folder_path(path: &str) -> Option<String> {
    get_config_dir().map_or_else(
        || None,
        |p| Some(format!("{}/{}", p.to_str().unwrap_or_default(), path)),
    )
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
        does_path_exists,
        get_folder_path,
        remove_folder,
    };

    #[test]
    fn find_folder_path() {
        let fname = "notp";
        let path = get_folder_path(&fname);
        assert_eq!(true, path.is_some());
    }

    #[test]
    fn folder_does_not_exist() {
        let fname = "Gibberish";
        let path = get_folder_path(&fname);
        assert_eq!(true, path.is_some());
    }

    #[test]
    fn path_does_not_exist() {
        assert_eq!(false, does_path_exists("PathThatDoesNotExists"));
    }

    #[test]
    fn path_does_exist() {
        create_folder("test").ok();
        assert_eq!(true, does_path_exists("test"));
        remove_folder("test").ok();
    }

    #[test]
    fn cannot_create_folder() {
        let resp = create_folder("");
        assert_eq!(true, resp.is_err());
    }

    #[test]
    fn delete_folder() {
        let fname = "FolderThatNotExists";
        if !does_path_exists(&fname) {
            create_folder(&fname).ok();
        }
        assert_eq!(true, does_path_exists(&fname));
        let result = remove_folder(&fname);
        assert_eq!(true, result.is_ok());
        assert_eq!(false, does_path_exists(&fname));
    }

    #[test]
    fn couldnt_delete_folder_that_does_not_exist() {
        let fname = "SomeGibberish";

        let result = remove_folder(fname);
        assert_eq!(true, result.is_err());
    }
}
