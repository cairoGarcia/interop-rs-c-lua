use std::ffi::{CStr, CString};
use std::io;
use std::io::Write;

macro_rules! flush {
    () => {
        io::stdout().flush().unwrap()
    };
}

macro_rules! c_to_string {
    ($s: expr) => {
        unsafe { CStr::from_ptr($s).to_string_lossy() }
    };
}

/* TODO!
 * Implement a Macro to use c's printf safely.
 */

fn input(s: &str) -> String {
    let stdin = io::stdin();

    loop {
        print!("{s}");
        flush!();

        let mut ipt = String::new();

        stdin.read_line(&mut ipt).expect("Not a String: get_string");

        let ipt = ipt.trim();

        if ipt.len() < 1 {
            continue;
        };

        return ipt.to_string();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_string(s: *const i8) -> *const i8 {
    loop {
        let ipt = input(&c_to_string!(s));

        let ipt: &str = ipt.trim();

        if ipt.len() == 0 {
            continue;
        };

        return CString::new(ipt).unwrap().into_raw();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_char(s: *const i8) -> i8 {
    loop {
        let ipt = input(&c_to_string!(s));

        if ipt.len() != 1 {
            continue;
        }

        /* gets first char */
        return ipt.chars().nth(0).expect("get_char error") as i8;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_i32(s: *const i8) -> i32 {
    loop {
        let ipt = input(&c_to_string!(s));

        let ipt: Result<i32, _> = ipt.parse();

        match ipt {
            Ok(val) => return val,
            Err(_) => continue,
        };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_u8(s: *const i8) -> u8 {
    loop {
        let ipt = input(&c_to_string!(s));

        let ipt: Result<u8, _> = ipt.parse();

        match ipt {
            Ok(val) => return val,
            Err(_) => continue,
        };
    }
}

#[unsafe(no_mangle)]
/* draws a square */
pub extern "C" fn square(size: u8)
{
    for _h in 0..size {
        for _w in 0..size {
            print!("#");
        }
        print!("\n");
    }
}
