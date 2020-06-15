
#[no_mangle]
pub extern "C" fn get_name() -> String {
    "adder".to_string()
}

#[no_mangle]
pub extern "C" fn get_code() -> String {
    "print('adder was initialized\\n');".to_string()
}

#[no_mangle]
pub extern "C" fn run(params_str: &str) -> Option<String> {
    println!("run {:?}", params_str);
    Some("response yeah".to_string())
}

// get_types ?
