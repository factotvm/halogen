use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

use crate::processor::{Processor, ProcessorError, ProcessorKind};

pub(crate) struct OrgProcessor {
    contents: BufReader<File>,
}

impl Processor for OrgProcessor {
    fn new(file: File) -> Self { OrgProcessor { contents: BufReader::new(file) } }
    fn kind(&self) -> ProcessorKind { ProcessorKind::Org }
    fn write_to(&mut self, path: &Path) -> Result<(), ProcessorError> {
        let mut file = File::create(path)?;
        let mut output = String::from("<!-- Org HTML generated by Halogen -->\n");
        self.contents.read_to_string(&mut output)?;
        file.write_all(output.as_ref())?;
        Ok(())
    }
}
