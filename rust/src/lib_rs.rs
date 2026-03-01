use std::ffi::{c_char, CStr, CString};
use std::io;

#[unsafe(no_mangle)]
pub extern "C" fn get_string() -> *const i8 {
    loop {
        let stdin = io::stdin();

        let mut ipt = String::new();
        stdin.read_line(&mut ipt).unwrap();

        let ipt: &str = ipt.trim();
        /* normal rust code above */

        if ipt.len() == 0 { continue };

        /* adds null terminator to string */
        let ipt: CString = CString::new(ipt).expect("CString panic");

        /* converts to C char pointer safer type */
        let ipt = ipt.into_raw();
        let ipt: *const i8 = ipt;
        return ipt;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_char() -> u8 {
    loop {
        let stdin = io::stdin();

        let mut ipt = String::new();
        stdin.read_line(&mut ipt).unwrap();
        let ipt: &str = ipt.trim();

        /* gets first char */
        let ipt = ipt.chars().nth(0);

        let ipt: u8 = match ipt {
            Some(val) => val as u8,
            _ => continue,
        };

        return match Some(ipt) {
            Some(char) => char,
            _ => continue,
        };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_int() -> i32 {
    loop {
        let stdin = io::stdin();

        let mut ipt = String::new();
        stdin.read_line(&mut ipt).unwrap();
        let ipt: &str = ipt.trim();

        let ipt: Result<i32, _> = ipt.parse();

        let ipt: i32 = match ipt {
            Ok(val) => val as i32,
            Err(_) => continue,
        };

        return match Some(ipt) {
            Some(char) => char,
            _ => continue,
        };
    }
}

#[unsafe(no_mangle)]
/* draws a square */
pub extern "C" fn square(size: u8) {
    for _h in 0..size {
        for _w in 0..size {
            print!("#");
        }
        print!("\n");
    }
}

#[unsafe(no_mangle)]
/* funnier print */
pub extern "C" fn stringout(s: *const c_char) {
    let s: String = unsafe { CStr::from_ptr(s).to_string_lossy().into_owned() };
    println!("{s}");
}
