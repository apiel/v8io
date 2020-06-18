use rusty_v8 as v8;
use std::env;

pub fn args(
    scope: v8::FunctionCallbackScope,
    _args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let context = scope.get_current_context().unwrap();

    let args: Vec<String> = env::args().collect();
    let len = args.len() as i32;
    let array = v8::Array::new(scope, len);
    for (i, arg) in args.iter().enumerate() {
        // println!("In position {} we have value {}", i, arg);
        array.set_index(
            context,
            i as u32,
            v8::String::new(scope, arg).unwrap().into(),
        );
    }
    rv.set(array.into());
}
