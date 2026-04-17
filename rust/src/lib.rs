use std::ffi::{CString, CStr};
use std::io;
use std::io::{Write};

macro_rules! flush {
    () => {
        io::stdout().flush().unwrap()
    }
}

macro_rules! c_to_string {
    ($s: expr) => {
        unsafe { CStr::from_ptr($s).to_string_lossy()}
    }
}

/* TODO!
 * Implement a Macro to use c's printf safely.
 */

#[unsafe(no_mangle)]
pub extern "C" fn get_string(s: *const i8) -> *const i8 {
    let stdin = io::stdin();
    loop {

        let mut ipt = String::new();

        print!("{}", c_to_string!(s));
        flush!();

        stdin.read_line(&mut ipt)
            .expect("Not a String: get_string");

        let ipt: &str = ipt.trim();

        if ipt.len() == 0 { continue };

        let ipt = CString::new(ipt).unwrap().into_raw();

        return ipt;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_char(s: *const i8) -> i8 {
    let stdin = io::stdin();

    loop {
        let mut ipt = String::new();

        print!("{}", c_to_string!(s));
        flush!();

        stdin.read_line(&mut ipt)
            .expect("Char error: get_char");

        let ipt: &str = ipt.trim();

        if ipt.len() > 1 {continue};

        /* gets first char */
        match ipt.chars().nth(0) {
            Some(val) => return val as i8,
            _ => continue,
        };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_i32(s: *const i8) -> i32 {
    let stdin = io::stdin();

    loop {
        let mut ipt = String::new();

        print!("{}", c_to_string!(s));
        flush!();
    
        stdin.read_line(&mut ipt).unwrap();
        let ipt: &str = ipt.trim();

        let ipt: Result<i32, _> = ipt.parse();

        match ipt {
            Ok(val) => return val,
            Err(_) => continue,
        };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_u8(s: *const i8) -> u8 {
    let stdin = io::stdin();

    loop {
        let mut ipt = String::new();

        print!("{}", c_to_string!(s));
        flush!();

        stdin.read_line(&mut ipt).unwrap();
        let ipt: &str = ipt.trim();

        let ipt: Result<u8, _> = ipt.parse();

        match ipt {
            Ok(val) => return val,
            Err(_) => continue,
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
