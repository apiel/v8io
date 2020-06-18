#[no_mangle]
fn get_name() -> String {
    "adder".to_string()
}

#[no_mangle]
fn get_code() -> String {
    "globalThis.adder = (value) => coreInstantiate('adder', value);".to_string()
}

#[no_mangle]
fn run(params_str: &str) -> Option<String> {
    println!("run {:?}", params_str);
    Some("response yeah".to_string())
}

#[no_mangle]
fn run_async(params_str: &str, cb: Box<dyn FnMut(Option<String>)>) {
    println!("run {:?}", params_str);
    let mut cb = cb;
    cb(Some("response yeah".to_string()));
}

// get_types ?
