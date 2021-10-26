use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr::null;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Int")]
    Int,
    #[error("Utf8Error")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("call extern function error: {0}")]
    Return(String),
}

pub trait ResultExt {
    fn to_const_char(&self) -> *mut c_char;
}

impl<T> ResultExt for Result<T> {
    fn to_const_char(&self) -> *mut c_char {
        if let Err(e) = self {
            return e.into_raw();
        }
        // null()
        CString::new("OK").unwrap().into_raw()
    }
}


// impl ResultExt for Error {
//     fn to_const_char(&self) -> *mut c_char {
//         let hello = CString::new(self.to_string()).unwrap();
//         unsafe {
//             CString::new(self).into_raw()
//         }
//     }
// }

// impl ResultExt for String {
//     fn to_const_char(&self) -> *mut c_char {
//         let hello = CString::new(self).unwrap();
//         unsafe {
//             CString::new(self).into_raw()
//         }
//     }
// }

// impl ResultExt for String {
//     fn to_const_char(&self) -> *mut c_char {
//         match CString::new(self) {
//             // todo:
//             Err(e) => CString::new(e).unwrap_err().as_ptr(),
//             Ok(s) => s.as_ptr(),
//         }
//     }
// }

pub trait ConstCCharExt {
    fn to_str(&self) -> Result<&str>;
}

impl ConstCCharExt for *const c_char {
    fn to_str(&self) -> Result<&str> {
        // 保持对外部字符串的借用
        // 最小化unsafe区域
        let res = unsafe { CStr::from_ptr(*self).to_str() };
        Ok(res?)
    }
}


pub trait ErrorExt {
    fn into_raw(&self) -> *mut c_char;
}

impl ErrorExt for Error {
    fn into_raw(&self) -> *mut c_char {
        let res = CString::new(self.to_string()).expect("CString new failed");
        res.into_raw()
    }
}

fn rust_init(config: &str) -> Result<()> {
    Ok(())
}


// 只是编译器 没检测出来，运行的时候还是错误的
fn init(config: *const c_char) -> *mut c_char {
    let config = config.to_str()?;

    // 奇怪，这里不报错
    rust_init(config)?;

    // null()
    CString::new("OK").unwrap().into_raw()
}

fn main() {
    init(null());
}