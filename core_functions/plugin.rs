use rusty_v8 as v8;

use crate::modules::plugin_loader;

pub fn core_instantiate(
    scope: v8::FunctionCallbackScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    assert!(args.length() == 1);
    let obj = args.get(0);

    if let Some(name) = obj.to_string(scope) {
        plugin_loader::instantiate(name.to_rust_string_lossy(scope));
    }
}
