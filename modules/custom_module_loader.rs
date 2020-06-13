use crate::modules::module;
use rusty_v8 as v8;
use std::convert::TryFrom;
use std::path::Path;

pub fn init_module_loader<'sc>(scope: &mut impl v8::ToLocal<'sc>, context: v8::Local<v8::Context>) {
    let path = match std::env::var("V8IO_MODULE_LOADER") {
        Ok(val) => val,
        Err(_) => {
            let file = std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join("module_loader.js");
            file.into_os_string().into_string().unwrap()
        }
    };
    if Path::new(&path).exists() {
        let contents = std::fs::read_to_string(path)
            .expect("Something went wrong while loading custom module loader.");
        let source = v8::String::new(scope, &contents).unwrap();
        let mut script = v8::Script::compile(scope, context, source, None).unwrap();
        script.run(scope, context).unwrap();
    }
}

pub fn get_module<'sc>(
    scope: &mut impl v8::ToLocal<'sc>,
    context: v8::Local<v8::Context>,
    specifier_str: String,
    referrer_str: String,
) -> Option<module::ModuleToLoad> {
    // here we could have a flag in custom_module_loader to know if it is activated

    let code = "typeof coreModuleLoader === 'function' && coreModuleLoader('".to_string()
        + &specifier_str
        + "','"
        + &referrer_str
        + "')";

    let source = v8::String::new(scope, &code).unwrap();
    let mut script = v8::Script::compile(scope, context, source, None).unwrap();
    let result = script.run(scope, context).unwrap();
    if result.is_string() {
        return get_module_from_string(scope, result);
    } else if result.is_array() {
        return get_module_from_array(scope, context, result);
    } else {
        return None;
    }
}

fn get_module_from_string<'sc>(
    scope: &mut impl v8::ToLocal<'sc>,
    result: v8::Local<v8::Value>,
) -> Option<module::ModuleToLoad> {
    let result = result.to_string(scope).unwrap();
    Some(module::ModuleToLoad {
        absolute_path: result.to_rust_string_lossy(scope),
        code: None,
    })
}

fn get_module_from_array<'sc>(
    scope: &mut impl v8::ToLocal<'sc>,
    context: v8::Local<v8::Context>,
    result: v8::Local<v8::Value>,
) -> Option<module::ModuleToLoad> {
    let array = v8::Local::<v8::Array>::try_from(result).unwrap();
    if array.length() == 2 {
        let absolute_path = array.get_index(scope, context, 0).unwrap();
        let code = array.get_index(scope, context, 1).unwrap();
        if absolute_path.is_string() && code.is_string() {
            return Some(module::ModuleToLoad {
                absolute_path: absolute_path
                    .to_string(scope)
                    .unwrap()
                    .to_rust_string_lossy(scope),
                code: Some(code.to_string(scope).unwrap().to_rust_string_lossy(scope)),
            });
        }
    }
    return None;
}
