use anyhow::{bail, Result};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: rawcopy.exe <file_path> <save_path>");
        eprintln!();
        eprintln!("'file_path' is the absolute path of the file must exist.");
        eprintln!("'save_path' is the directory where the copied file will be saved.\n\tThe directory must exist, and the file must not already exist.\n\tThe file name will be the same as the name of the file being copied.\n\tIf it points to an NTFS filesystem image, then a suffix will be appended.\n");
        eprintln!("Under Windows and when run with administrative privileges\n");
        eprintln!(r#"eg. rawcopy.exe "C:\swapfile.sys" d:\tmp"#);
        bail!("Aborted, Missing argument.\n");
    }

    let file_path = args[1].as_str();
    let save_path = args[2].as_str();
    rawcopy_rs::rawcopy(file_path, save_path)?;
    Ok(())
}
