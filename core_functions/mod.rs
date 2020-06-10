use rusty_v8 as v8;

mod print;

pub fn init<'sc>(
    scope: &'sc mut impl v8::ToLocal<'sc>,
) -> (
    v8::Local<'_, v8::ObjectTemplate>,
    &'sc mut impl v8::ToLocal<'sc>,
) {
    let object_templ = v8::ObjectTemplate::new(scope);
    let function_templ = v8::FunctionTemplate::new(scope, fortytwo_callback);
    let name = v8::String::new(scope, "yo").unwrap();
    object_templ.set(name.into(), function_templ.into());

    let function_templ = v8::FunctionTemplate::new(scope, print::print);
    let name = v8::String::new(scope, "print").unwrap();
    object_templ.set(name.into(), function_templ.into());

    (object_templ, scope)
}

fn fortytwo_callback(
    scope: v8::FunctionCallbackScope,
    _: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    rv.set(v8::Integer::new(scope, 42).into());
}
