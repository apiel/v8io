use crate::modules::default_loader;
use crate::modules::module;
use rusty_v8 as v8;

extern crate libloading as lib;

type AddFunc = unsafe fn(isize, isize) -> isize;

pub fn load_plugin<'sc>(
    _scope: &mut impl v8::ToLocal<'sc>,
    _context: v8::Local<v8::Context>,
    specifier_str: String,
    referrer_str: String,
) -> module::Module {
    let absolute_path = default_loader::get_module(specifier_str, referrer_str).absolute_path;

    let lib = lib::Library::new(absolute_path.clone()).unwrap();
    unsafe {
        let func: lib::Symbol<AddFunc> = lib.get(b"add").unwrap();
        let answer = func(1, 2);
        println!("1 + 2 = {}", answer);
    }

    module::Module {
        absolute_path,
        code: "".to_string(),
    }
}
