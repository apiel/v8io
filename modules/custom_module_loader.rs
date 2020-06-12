use rusty_v8 as v8;
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
        let contents =
            std::fs::read_to_string(path).expect("Something went wrong reading the file");
        let source = v8::String::new(scope, &contents).unwrap();
        let mut script = v8::Script::compile(scope, context, source, None).unwrap();
        script.run(scope, context).unwrap();
    }
}

pub fn get_specifier_path<'sc>(
    scope: &mut impl v8::ToLocal<'sc>,
    context: v8::Local<v8::Context>,
    specifier_str: String,
    referrer_str: String,
) -> Option<String> {
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
        let result = result.to_string(scope).unwrap();
        return Some(result.to_rust_string_lossy(scope));
    } else {
        return None;
    }
}
