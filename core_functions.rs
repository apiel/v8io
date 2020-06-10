use rusty_v8 as v8;

pub fn init<'sc>(
    scope: &'sc mut impl v8::ToLocal<'sc>,
) -> rusty_v8::scope::Scope<rusty_v8::scope::ContextScope, impl v8::ToLocal<'sc>> {
    let object_templ = v8::ObjectTemplate::new(scope);
    let function_templ = v8::FunctionTemplate::new(scope, fortytwo_callback);
    let name = v8::String::new(scope, "yo").unwrap();
    object_templ.set(name.into(), function_templ.into());

    let function_templ = v8::FunctionTemplate::new(scope, print);
    let name = v8::String::new(scope, "print").unwrap();
    object_templ.set(name.into(), function_templ.into());

    let context = v8::Context::new_from_template(scope, object_templ);
    v8::ContextScope::new(scope, context)
}

// pub fn init<'sc>(
//     scope: &'sc mut impl v8::ToLocal<'sc>,
// ) -> rusty_v8::scope::Scope<rusty_v8::scope::ContextScope, impl v8::ToLocal<'sc>> {
//     let object_templ = v8::ObjectTemplate::new(scope);
//     let function_templ = v8::FunctionTemplate::new(scope, fortytwo_callback);
//     let name = v8::String::new(scope, "yo").unwrap();
//     object_templ.set(name.into(), function_templ.into());

//     let function_templ = v8::FunctionTemplate::new(scope, print);
//     let name = v8::String::new(scope, "print").unwrap();
//     object_templ.set(name.into(), function_templ.into());

//     let context = v8::Context::new_from_template(scope, object_templ);
//     v8::ContextScope::new(scope, context)
// }

fn fortytwo_callback(
    scope: v8::FunctionCallbackScope,
    _: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    rv.set(v8::Integer::new(scope, 42).into());
}

fn print(
    scope: v8::FunctionCallbackScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    let arg_len = args.length();
    assert!(arg_len >= 0 && arg_len <= 2);
    let obj = args.get(0);
    let is_err_arg = args.get(1);
    let mut hs = v8::HandleScope::new(scope);
    let scope = hs.enter();
    let mut is_err = false;
    if arg_len == 2 {
        let int_val = is_err_arg
            .integer_value(scope)
            .expect("Unable to convert to integer");
        is_err = int_val != 0;
    };
    let mut try_catch = v8::TryCatch::new(scope);
    let _tc = try_catch.enter();
    let str_ = match obj.to_string(scope) {
        Some(s) => s,
        None => v8::String::new(scope, "").unwrap(),
    };
    if is_err {
        eprint!("{}", str_.to_rust_string_lossy(scope));
    } else {
        print!("{}", str_.to_rust_string_lossy(scope));
    }
}
