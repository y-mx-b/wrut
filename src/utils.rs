use std::env::current_dir;
use std::path::PathBuf;
use wrut::WrutError;

pub fn check_path(p: Option<PathBuf>) -> Result<PathBuf, WrutError> {
    let path = p.unwrap_or(current_dir()?).canonicalize()?;
    if !path.is_dir() {
        return Err(WrutError::ExpectedDirectory(path));
    }

    Ok(path)
}

pub fn check_name(n: Option<String>, p: &PathBuf) -> Result<String, WrutError> {
    match n {
        Some(str) => Ok(str),
        None => {
            let dir_name = p
                .file_name()
                .expect("canonicalized path and check if directory");
            if let Some(str) = dir_name.to_str() {
                Ok(str.to_string())
            } else {
                Err(WrutError::ExpectedUtf8(dir_name.to_os_string()))
            }
        }
    }
}
