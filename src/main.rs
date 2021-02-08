use glob::glob;
use std::alloc::System;
use std::env::current_dir;
use std::os::windows::ffi::OsStrExt;
use std::{intrinsics::copy_nonoverlapping, mem::size_of};

use bindings::{
    windows::win32::data_exchange::CloseClipboard, windows::win32::data_exchange::EmptyClipboard,
    windows::win32::data_exchange::OpenClipboard, windows::win32::data_exchange::SetClipboardData,
    windows::win32::shell::DROPFILES, windows::win32::windows_and_messaging::HWND, windows::BOOL,
};

pub const CF_HDROP: u32 = 15;
#[global_allocator]
static GLOBAL: System = System;

const USAGE: &str = r#"
Usage: pickup [file patterns]

Exameple: 
    # copy single file
    pickup Cargo.toml
    
    # copy all exe file in target subdirectories
    pickup "target/**/*.exe"

    # copy folder
    pickup src
"#;
fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 2 {
        println!("{}", USAGE);
        return;
    }
    let mut entries = vec![];
    let cur_dir = current_dir().expect("Get current dir error");
    for i in 1..argv.len() {
        let pattern = &argv[i];
        for entry in glob(pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    println!("copied {:?}", &path.display());
                    let target = cur_dir.join(&path);
                    entries.push(target.into_os_string());
                }
                Err(e) => {
                    println!("Fail to copy {:?}", e);
                }
            }
        }
    }
    if entries.len() == 0 {
        println!("[-] The file you specified cannot be found");
        return;
    }
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

#[cfg(test)]
mod tests {
    use glob::glob;
    #[test]
    fn test_glob() {
        for entry in glob("target/**/*.exe").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => println!("{:?}", path.display()),
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
