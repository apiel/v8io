use crate::script_origin;

use rusty_v8 as v8;

pub fn resolver<'a>(
    context: v8::Local<'a, v8::Context>,
    specifier: v8::Local<'a, v8::String>,
    _referrer: v8::Local<'a, v8::Module>,
) -> Option<v8::Local<'a, v8::Module>> {
    let mut cbs = v8::CallbackScope::new_escapable(context);
    let mut hs = v8::EscapableHandleScope::new(cbs.enter());
    let scope = hs.enter();
    let specifier_str = specifier.to_rust_string_lossy(scope);
    println!("specifier_str {:?}", specifier_str);
    let origin = script_origin::script_origin(scope, "module.js");
    let source = v8::script_compiler::Source::new(specifier, &origin);
    let module = v8::script_compiler::compile_module(scope, source).unwrap();
    Some(scope.escape(module))
}
