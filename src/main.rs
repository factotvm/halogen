use std::fmt::Error;
use std::fs;
use std::path::Path;

const SRC_DIR: &str = "content";
const DEST_DIR: &str = "public";

fn main() {
    if check_dir(SRC_DIR, false).is_ok() {
        println!("Using source directory: {}.", SRC_DIR)
    } else {
        panic!("The source directory ({}) is not present.", SRC_DIR)
    }

    if check_dir(DEST_DIR, true).is_ok() {
       println!("Using destination directory: {}.", DEST_DIR);
    } else {
        panic!("The destination directory could not be created ({}).", DEST_DIR)
    }
}

fn check_dir(path: &str, create_if_needed: bool) -> Result<(), Error> {
    if !Path::new(path).is_dir() {
        if create_if_needed {
            println!("Creating directory: {}", path);
            fs::create_dir_all(path).unwrap()
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};
    use super::*;

    #[test]
    fn check_dir_exist() {
        let src = create_path();
        fs::create_dir_all(src.clone()).expect("temporary file should be created");
        assert_eq!(check_dir(&src, false), Ok(()));
    }

    #[test]
    fn check_dir_will_not_create_dir() {
        let src = create_path();
        assert_eq!(check_dir(&src, false), Ok(()));
        assert!(!Path::new(&src).is_dir());
    }

    #[test]
    fn check_dir_will_create_dir() {
        let src = create_path();
        assert_eq!(check_dir(&src, true), Ok(()));
        assert!(Path::new(&src).is_dir());
    }

    fn create_path() -> String {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => format!("/tmp/src-{}", n.as_nanos()),
            _ => {panic!("unable to create timestamp for test path")}
        }
    }
}