use rusty_v8 as v8;

use crate::modules::plugin_loader;

pub fn core_instantiate(
    scope: v8::FunctionCallbackScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    assert!(args.length() == 1 || args.length() == 2);
    let obj_name = args.get(0);
    let obj_params = args.get(1);

    if let Some(name) = obj_name.to_string(scope) {
        let params_str: Option<String> = match obj_params.to_string(scope) {
            Some(s) => Some(s.to_rust_string_lossy(scope)),
            None => None,
        };
        plugin_loader::instantiate(name.to_rust_string_lossy(scope), params_str);
    }
}
