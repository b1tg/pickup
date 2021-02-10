use crate::Clipboard;
use anyhow::Result;
use cocoa::appkit::NSPasteboard;
use cocoa::foundation::{NSString, NSURL};
use cocoa::{base::nil, foundation::NSArray};
use std::{ffi::OsString, vec};
pub struct OSXClipboard {
    entries: Vec<OsString>,
}
impl Clipboard for OSXClipboard {
    fn new(entries: Vec<OsString>) -> Result<OSXClipboard> {
        Ok(OSXClipboard { entries })
    }
    fn copy_files(&self) -> Result<()> {
        let mut ns_urls = vec![];
        let paste_board = unsafe { NSPasteboard::generalPasteboard(nil) };
        for entry in self.entries.clone() {
            let entry_str = entry.into_string().unwrap();
            let path = unsafe { NSString::alloc(nil).init_str(&entry_str) };
            let ns_url = unsafe { NSURL::alloc(nil).initFileURLWithPath_(path) };
            ns_urls.push(ns_url);
        }
        unsafe {
            let objs = NSArray::arrayWithObjects(nil, &ns_urls);
            paste_board.clearContents();
            paste_board.writeObjects(objs);
        }
        Ok(())
    }
}
