use std::ffi::{c_char, CString, CStr};
use std::io;

#[unsafe(no_mangle)]
pub extern "C" fn get_string() -> *const c_char {
    let stdin = io::stdin();

    let mut ipt = String::new();
    stdin.read_line(&mut ipt).unwrap();

    let ipt: &str = ipt.trim();
    /* normal rust code above */

    /* adds null terminator to string */
    let ipt: CString = CString::new(ipt).expect("CString panic");

    /* converts to C char pointer safer type. 
     * somehow c_char == i8, not u8. (????)
     */
    let ipt = ipt.into_raw();
    let ipt: *const c_char = ipt;
    ipt
}

#[unsafe(no_mangle)]
pub extern "C" fn square(size: i32) {
    for _h in 0..size {
        for _w in 0..size {
            print!("#");
        };
        print!("\n");
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn writeout(s: *const c_char) {
    let s: String = unsafe {
        CStr::from_ptr(s).to_string_lossy().into_owned()
    };
    println!("{s}");
}
