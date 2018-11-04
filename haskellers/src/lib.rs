use std::ffi::CStr;
use std::os::raw::c_char;

/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/

#[no_mangle]
pub extern fn double_input(x: i32) -> i32 {
    2 * x
}

#[no_mangle]
pub extern fn fact(x: i32) -> i32 {
    match x {
        0 => 1,
        x => x * fact(x - 1),
    }
}

#[no_mangle]
pub extern fn print_string(x: *const c_char) {
  unsafe {
    let cstring = CStr::from_ptr(x);
    if let Ok(input) = cstring.to_str() {
      println!("{}", input);
    } else {
      panic!("Unable to print input");
    }
  }
}
