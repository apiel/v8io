
#[no_mangle]
pub extern "C" fn get_name() -> String {
    "adder".to_string()
}

#[no_mangle]
pub extern "C" fn get_code() -> String {
    "print('adder was initialized\\n');".to_string()
}

#[no_mangle]
pub extern "C" fn run() -> isize {
    4
}
// pub extern "C" fn run(params_str: Option<String>) -> isize {
//     // if let Some(params) = params_str {
//     //     println!("run {:?}", params.clone());
//     // };
//     4
//     // a + b
// }

// get_types ?
