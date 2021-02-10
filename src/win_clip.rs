use std::alloc::System;
use std::os::windows::ffi::OsStrExt;
use std::{intrinsics::copy_nonoverlapping, mem::size_of};

use bindings::{
    windows::win32::data_exchange::CloseClipboard, windows::win32::data_exchange::EmptyClipboard,
    windows::win32::data_exchange::OpenClipboard, windows::win32::data_exchange::SetClipboardData,
    windows::win32::shell::DROPFILES, windows::win32::windows_and_messaging::HWND, windows::BOOL,
};

use crate::Clipboard;
pub struct WinClipboard {
    entries: Vec<OsString>,
}
pub const CF_HDROP: u32 = 15;
#[global_allocator]
static GLOBAL: System = System;

use crate::Clipboard;
use anyhow::Result;
pub struct WinClipboard {
    entries: Vec<OsString>,
}
impl Clipboard for WinClipboard {
    fn new(entries: Vec<OsString>) -> Result<WinClipboard> {
        Ok(WinClipboard { entries })
    }
    fn copy_files(&self) -> Result<()> {
        let mut clip_buf: Vec<u16> = vec![];
        for entry in &self.entries {
            let mut result: Vec<u16> = entry.encode_wide().collect();
            clip_buf.append(&mut result);
            clip_buf.push(0);
        }
        clip_buf.push(0);
        let p_files = size_of::<DROPFILES>();
        let mut h_global = vec![0u8; clip_buf.len() * 2 + p_files];
        let dropfiles: *mut DROPFILES = h_global.as_mut_ptr() as *mut DROPFILES;
        let buf_ptr = clip_buf.as_ptr();
        unsafe {
            (*dropfiles).p_files = p_files as _;
            (*dropfiles).f_wide = BOOL(1);
            copy_nonoverlapping(
                buf_ptr,
                h_global.as_mut_ptr().offset(p_files as _) as *mut u16,
                clip_buf.len(),
            );
            let h_mem = core::mem::transmute(h_global.as_mut_ptr());
            OpenClipboard(HWND(0));
            EmptyClipboard();
            CloseClipboard();

            OpenClipboard(HWND(0));
            SetClipboardData(CF_HDROP, h_mem);
            CloseClipboard();
        }
    }
}

pub fn win_pickup(entries: Vec<OsString>) {
    let mut clip_buf: Vec<u16> = vec![];
    for entry in &entries {
        let mut result: Vec<u16> = entry.encode_wide().collect();
        clip_buf.append(&mut result);
        clip_buf.push(0);
    }
    clip_buf.push(0);
    let p_files = size_of::<DROPFILES>();
    let mut h_global = vec![0u8; clip_buf.len() * 2 + p_files];
    let dropfiles: *mut DROPFILES = h_global.as_mut_ptr() as *mut DROPFILES;
    let buf_ptr = clip_buf.as_ptr();
    unsafe {
        (*dropfiles).p_files = p_files as _;
        (*dropfiles).f_wide = BOOL(1);
        copy_nonoverlapping(
            buf_ptr,
            h_global.as_mut_ptr().offset(p_files as _) as *mut u16,
            clip_buf.len(),
        );
        let h_mem = core::mem::transmute(h_global.as_mut_ptr());
        OpenClipboard(HWND(0));
        EmptyClipboard();
        CloseClipboard();

        OpenClipboard(HWND(0));
        SetClipboardData(CF_HDROP, h_mem);
        CloseClipboard();
    }
}
