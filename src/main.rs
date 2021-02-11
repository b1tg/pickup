use glob::glob;
use std::env::current_dir;
mod common;
pub use common::Clipboard;

#[cfg(target_os = "macos")]
mod osx_clip;
#[cfg(target_os = "macos")]
pub type Clip = osx_clip::OSXClipboard;

#[cfg(target_os = "windows")]
mod win_clip;
#[cfg(target_os = "windows")]
pub type Clip = win_clip::WinClipboard;

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
    let clip = Clip::new(entries).unwrap();
    let _ = clip.copy_files();
}

#[cfg(test)]
mod tests {
    use glob::glob;
    #[test]
    fn test_glob() {}

    #[test]
    fn test_obj() {
        // let pb: NSPasteboard.general;
    }
}
