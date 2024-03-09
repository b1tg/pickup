use crate::Clipboard;
use anyhow::anyhow;
use anyhow::Result;
use std::ffi::OsString;
use std::process::Command;
pub struct LinuxClipboard {
    entries: Vec<OsString>,
}
impl Clipboard for LinuxClipboard {
    fn new(entries: Vec<OsString>) -> Result<LinuxClipboard> {
        Ok(LinuxClipboard { entries })
    }
    // xclip-copyfile LICENSE README.md
    // xclip-pastefile
    fn copy_files(&self) -> Result<()> {
        let status = Command::new("xclip-copyfile")
            .args(self.entries.clone())
            .status()
            .expect("Failed to execute xclip-copyfile");
        if status.success() {
            Ok(())
        } else {
            Err(anyhow!("status: {status}"))
        }
    }
}
