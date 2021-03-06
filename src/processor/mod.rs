use std::convert::Infallible;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::processor::css_processor::CssProcessor;
use crate::processor::md_processor::MdProcessor;
use crate::processor::none_processor::NoneProcessor;
use crate::processor::org_processor::OrgProcessor;

mod css_processor;
mod md_processor;
mod none_processor;
mod org_processor;

/// Represents a type that can process a given file for the web.
trait Processor {
    /// Creates a new processor for the given file.
    ///
    /// # Arguments
    ///
    /// * `file`: The file to process.
    ///
    /// returns: A Processor implementation.
    fn new(file: File) -> Self where Self: Sized;

    /// A want-to-be field for the type of processor.
    ///
    /// Ideally, this would be a simple property, as the look- up time is
    /// constant. However, Rust does not yet support trait fields. You can read
    /// more about that work at: https://github.com/rust-lang/rfcs/pull/1546
    ///
    /// It is also highly likely that I don't know what I'm doing, and there is
    /// a better way.
    ///
    /// returns: The concrete type of processor.
    fn kind(&self) -> ProcessorKind;

    /// Performs the processing of the file and writes the result.
    ///
    /// # Arguments
    ///
    /// * `path`: Where the results will be written.
    ///
    /// returns: A result indicating the outcome.
    fn write_to(&mut self, path: &Path) -> Result<(), ProcessorError>;
}

#[derive(Debug, PartialEq)]
struct ProcessorError {
    kind: ProcessorErrorKind,
    path: String,
}

#[derive(Debug, PartialEq)]
enum ProcessorErrorKind {
    Unknown,
}

impl From<std::io::Error> for ProcessorError {
    fn from(error: Error) -> Self {
        ProcessorError {
            kind: ProcessorErrorKind::Unknown,
            // FIXME: this isn't a path...
            path: String::from(format!("{:?}", error))
        }
    }
}

impl Display for ProcessorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProcessorError {{ path: {} }}", self.path)
    }
}

/// The type of file we are processing (named after the extension).
#[derive(Debug, PartialEq)]
enum ProcessorKind {
    None,
    Css,
    Md,
    Org,
}

impl FromStr for ProcessorKind {
    type Err = Infallible;

    /// This will never return error result because Self::None captures it.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "css" => Ok(Self::Css),
            "md" => Ok(Self::Md),
            "org" => Ok(Self::Org),
            _ => Ok(Self::None),
        }
    }
}

struct ProcessorFactory {}

impl ProcessorFactory {
    fn new() -> Self {
        ProcessorFactory {}
    }

    fn make(&self, path: &PathBuf) -> Box<dyn Processor> {
        let extension = path.extension().unwrap_or(&OsStr::new("")).to_str().unwrap_or("");
        let file = File::open(path.as_path()).unwrap_or_else(|_| panic!("file failed: {:?}", path));
        match ProcessorKind::from_str(extension).unwrap_or(ProcessorKind::None) {
            ProcessorKind::None => Box::new(NoneProcessor::new(file)),
            ProcessorKind::Css => { Box::new(CssProcessor::new(file)) }
            ProcessorKind::Md => { Box::new(MdProcessor::new(file)) }
            ProcessorKind::Org => { Box::new(OrgProcessor::new(file)) }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    #[test]
    #[should_panic]
    fn unreadable_file_panics() {
        let factory = ProcessorFactory::new();
        let _ = factory.make(&PathBuf::from("/tmp/does-not-exist.txt"));
    }

    #[test]
    fn unhandled_extension_returns_none_processor() {
        let none_path = create_file_with_extension("txt");
        let processor = ProcessorFactory::new().make(&none_path);
        assert_eq!(processor.kind(), ProcessorKind::None);
    }

    #[test]
    fn css_extension_returns_css_processor() {
        let css_path = create_file_with_extension("css");
        let processor = ProcessorFactory::new().make(&css_path);
        assert_eq!(processor.kind(), ProcessorKind::Css);
    }

    #[test]
    fn md_extension_returns_md_processor() {
        let md_path = create_file_with_extension("md");
        let processor = ProcessorFactory::new().make(&md_path);
        assert_eq!(processor.kind(), ProcessorKind::Md);
    }

    #[test]
    fn org_extension_returns_org_processor() {
        let org_path = create_file_with_extension("org");
        let processor = ProcessorFactory::new().make(&org_path);
        assert_eq!(processor.kind(), ProcessorKind::Org)
    }

    fn create_file_with_extension(extension: &str) -> PathBuf {
        let path = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => PathBuf::from(format!("/tmp/src-{}.{}", n.as_nanos(), extension)),
            _ => { panic!("unable to create timestamp for test path") }
        };
        File::create(path.clone()).expect("file cannot be created");
        path
    }
}
