<h1 align="center" style="border-bottom: none">
    RawCopy-rs</br>
</h1>

<p align="center">
  <a href="https://github.com/pkptzx/rawcopy-rs"><img src="https://img.shields.io/github/stars/pkptzx/rawcopy-rs"></a>
  <a href="https://github.com/pkptzx/rawcopy-rs"><img src="https://img.shields.io/docsrs/rawcopy-rs"></a> 
  <a href="https://github.com/pkptzx/rawcopy-rs"><img src="https://img.shields.io/crates/d/rawcopy-rs"></a>  
  <a href="https://github.com/pkptzx/rawcopy-rs"><img src="https://img.shields.io/github/license/pkptzx/rawcopy-rs"></a>
</p>

`RawCopy` crate provides the capability to use "Volume Shadow Copy technology" for file copying in Rust.  
Primarily aimed at replicating files that cannot be directly copied due to being in use.

## Usage
`RawCopy` must be run with Administrator privileges on Windows.  

### Download Prebuilt Binaries 
[Release download](https://github.com/pkptzx/rawcopy-rs/releases/latest)  

Usage: rawcopy.exe <file_path> <save_path>

    `file_path` is the absolute path of the file must exist.  
    `save_path` is the directory where the copied file will be saved.  
            The directory must exist, and the file must not exist.  
            The file name will be the same as the name of the file being copied.  
            If it points to an NTFS filesystem image, then a suffix will be appended.

example:  
```powershell 
rawcopy.exe "C:\swapfile.sys" d:\tmp
```
### Use rawcopy-rs crate in Rust
#### 1. Add tui-realm to your Cargo.toml
```toml
[dependencies]
rawcopy-rs = "1.0.80"
```
#### 2. use api to copy file
```rust
rawcopy_rs::rawcopy(r"C:\swapfile.sys", ".");
```

## Build from source
```shell
cargo build --release
```

# License
MIT

# Acknowledgements
The [ntfs](https://github.com/ColinFinck/ntfs) A low-level NTFS filesystem library implemented in Rust.
