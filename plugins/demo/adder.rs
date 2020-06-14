#[no_mangle]
pub extern "C" fn get_name() -> String {
    "adder".to_string()
}

#[no_mangle]
pub extern "C" fn get_code() -> String {
    "print('adder was initialized\\n');".to_string()
}

#[no_mangle]
pub extern "C" fn run(a: isize, b: isize) -> isize {
    a + b
}

// get_types ?
