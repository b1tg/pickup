use anyhow::Result;
use std::ffi::OsString;
pub trait Clipboard: Sized {
    fn new(entries: Vec<OsString>) -> Result<Self>;
    fn copy_files(&self) -> Result<()>;
}
