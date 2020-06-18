#[no_mangle]
fn get_name() -> String {
    "hello".to_string()
}

#[no_mangle]
fn get_code() -> String {
    "globalThis.hello = (value) => coreInstantiate('hello', value);".to_string()
}

#[no_mangle]
fn run(params_str: &str) -> Option<String> {
    println!("run {:?}", params_str);
    Some("some sync data to return".to_string())
}

#[no_mangle]
fn run_async(params_str: &str, cb: Box<dyn FnMut(Option<String>)>) {
    println!("run {:?}", params_str);
    let mut cb = cb;
    cb(Some("some async data to return".to_string()));
}

// get_types ?
