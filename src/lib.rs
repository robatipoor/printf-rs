use lazy_static::lazy_static;
use libc::{c_char, c_int, FILE};

extern "C" {

    #[allow(dead_code)]
    pub fn printf(__restrict__fmt: *const c_char, ...) -> c_int;

    #[allow(dead_code)]
    pub fn setlocale(__category: c_int, __locale: *const c_char) -> c_char;

    #[allow(dead_code)]
    pub fn fflush(__stream: *mut FILE) -> c_int;

    #[allow(dead_code)]
    pub static stdout: *mut libc::FILE;
}

const LC_ALL: c_int = 6;

#[macro_export]
macro_rules! cstr {
    ($arg:expr) => {{
        let st = concat!($arg, "\0");
        let (ptr, _): (*const libc::c_char, usize) = core::mem::transmute(st);
        ptr
    }};
}

lazy_static! {
    pub static ref enable_utf8: () = {
        unsafe {
            setlocale(LC_ALL, cstr!("en_US.utf8"));
        }
    };
}

#[macro_export]
macro_rules! printf {
	($st:expr, $($args:expr),*) => (unsafe {
        let _ = *enable_utf8;
		printf(cstr!($st), $($args),*);
        fflush(stdout);
	})
}
