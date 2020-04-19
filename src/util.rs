use dirs;
use std::fs;
use std::path::{
    Path,
    PathBuf,
};

/// Checks if the given path exists or not.
pub fn is_path_exists(path: &str) -> bool {
    if let Some(p) = get_config_dir() {
        let totp_path = format!("{}/{}", p.to_str().unwrap(), path);
        return Path::new(&totp_path).exists();
    }
    false
}

fn get_config_dir() -> Option<PathBuf> {
    dirs::config_dir()
}

/// Creates folder inside the config folder.
pub fn create_folder(path: &str) {
    if !is_path_exists(path) {
        if let Some(p) = get_config_dir() {
            let totp_path = format!("{}/{}", p.to_str().unwrap(), path);
            let _ = fs::create_dir_all(&totp_path).is_ok();
        }
    }
}

/// Returns to the full path of the given path.
///
/// Let's say we want to get totp folder's full path. All we need to do is
/// saying `get_folder_path("totp")` and this function will return to the
/// full path such as `/home/user/.config/totp`.
pub fn get_folder_path(path: &str) -> Option<String> {
    if let Some(p) = get_config_dir() {
        let folder_path = format!("{}/{}", p.to_str().unwrap(), path);
        return Some(folder_path);
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::util::{
        create_folder,
        get_folder_path,
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
    fn should_create_totp_folder() {
        create_folder("totp");
        assert_eq!(true, is_path_exists("totp"));
    }
}
