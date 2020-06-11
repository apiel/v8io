use rusty_v8 as v8;

pub fn print(
    scope: v8::FunctionCallbackScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    print!("{}", get_text(scope, args));
}

pub fn eprint(
    scope: v8::FunctionCallbackScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    eprint!("{}", get_text(scope, args));
}

fn get_text(scope: v8::FunctionCallbackScope, args: v8::FunctionCallbackArguments) -> String {
    let arg_len = args.length();
    assert!(arg_len == 1);
    let obj = args.get(0);
    let mut hs = v8::HandleScope::new(scope);
    let scope = hs.enter();

    let mut try_catch = v8::TryCatch::new(scope);
    let _tc = try_catch.enter();
    let str_ = match obj.to_string(scope) {
        Some(s) => s,
        None => v8::String::new(scope, "").unwrap(),
    };

    str_.to_rust_string_lossy(scope)
}
