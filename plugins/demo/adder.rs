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

pub struct Cb {
    pub value: String,
}
impl Cb {
    pub fn callback(&mut self, _: Option<String>) {}
}

#[no_mangle]
pub extern "C" fn run_async(params_str: &str, cb: *mut Cb) {
    println!("run_async: call run_async in adder");
    // if !cb.is_null() {
        unsafe {
            println!("run_async: enter unsafe");

            let cb = &mut *cb;
            println!("run_async: run {:?}", params_str);
            println!("run_async: value {:?}", cb.value);
            cb.callback(Some("response yeah".to_string()));
            println!("run_async: done");
        }
    // }
}

// // pub extern "C" fn run_async(params_str: &str, cb: impl Fn(Option<String>))
// #[no_mangle]
// pub extern "C" fn run_async(params_str: &str, cb: unsafe extern "C" fn(Option<String>)) {
//     println!("run {:?}", params_str);
//     unsafe {
//         cb(Some("response yeah".to_string()));
//     }
// }

// get_types ?
