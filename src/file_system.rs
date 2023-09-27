use core::ffi::c_void;

use alloc::{ffi::CString, string::String};
use libc;

pub enum FileOpenMode {
    Read,
    NewFileWrite,
    Append,
    ReadWrite,
    NewFileReadWrite,
    ReadAppend,
}

impl From<FileOpenMode> for CString {
    fn from(value: FileOpenMode) -> Self {
        match value {
            FileOpenMode::Read => CString::new("r").unwrap(),
            FileOpenMode::NewFileWrite => CString::new("w").unwrap(),
            FileOpenMode::Append => CString::new("a").unwrap(),
            FileOpenMode::ReadWrite => CString::new("r+").unwrap(),
            FileOpenMode::NewFileReadWrite => CString::new("w+").unwrap(),
            FileOpenMode::ReadAppend => CString::new("a+").unwrap(),
        }
    }
}

// trait FileSystem {
//     fn open(file_name: &str, mode: FileOpenMode) -> Self;
//     fn read(self) -> String;
// }

// impl FileSystem for *mut libc::FILE {
//     fn open(file_name: &str, mode: FileOpenMode) -> Self {
//         let file = unsafe {
//             let directory = CString::new(file_name).unwrap();
//             let mode = CString::from(mode);

//             libc::fopen(directory.as_ptr() as *const u8, mode.as_ptr() as
// *const u8)         };
//         file
//     }

//     fn read(self) -> String {
//         let contents = unsafe {
//             let buf: c_void = CAr;
//             libc::fread();
//             todo!()
//         };
//         contents
//     }
// }
