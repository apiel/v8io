use rusty_v8 as v8;
use std::path::Path;

mod module_map;

pub fn insert(module: v8::Local<v8::Module>, absolute_path: String) {
    let module_item = module_map::ModuleItem { absolute_path };
    module_map::insert(module.get_identity_hash(), module_item);
}

pub fn resolver<'a>(
    context: v8::Local<'a, v8::Context>,
    specifier: v8::Local<'a, v8::String>,
    referrer: v8::Local<'a, v8::Module>,
) -> Option<v8::Local<'a, v8::Module>> {
    let mut cbs = v8::CallbackScope::new_escapable(context);
    let mut hs = v8::EscapableHandleScope::new(cbs.enter());
    let scope = hs.enter();
    let specifier_str = specifier.to_rust_string_lossy(scope);

    let referrer_path = module_map::get_absolute_path(referrer.get_identity_hash());
    let specifier_path = Path::new(&referrer_path)
        .parent()
        .unwrap()
        .join(specifier_str)
        .as_path()
        .display()
        .to_string();

    // println!("specifier_path {:?}", specifier_path);
    let module = compile_file(scope, &specifier_path).unwrap();
    Some(scope.escape(module))
}

pub fn compile_file<'sc>(
    scope: &mut impl v8::ToLocal<'sc>,
    file: &str,
) -> Option<v8::Local<'sc, v8::Module>> {
    let contents =
        std::fs::read_to_string(file.clone()).expect("Something went wrong reading the file");
    let source_string = v8::String::new(scope, &contents).unwrap();
    let module = compile(scope, file, source_string);

    insert(module.clone().unwrap(), file.to_string());

    module
}

fn compile<'sc>(
    scope: &mut impl v8::ToLocal<'sc>,
    file: &str,
    source_string: v8::Local<v8::String>,
) -> Option<v8::Local<'sc, v8::Module>> {
    let origin = script_origin(scope, file);
    let source = v8::script_compiler::Source::new(source_string, &origin);
    v8::script_compiler::compile_module(scope, source)
}

fn script_origin<'sc>(
    scope: &mut impl v8::ToLocal<'sc>,
    resource_name_: &str,
) -> v8::ScriptOrigin<'sc> {
    let resource_name = v8::String::new(scope, resource_name_).unwrap();
    let resource_line_offset = v8::Integer::new(scope, 0);
    let resource_column_offset = v8::Integer::new(scope, 0);
    let resource_is_shared_cross_origin = v8::Boolean::new(scope, true);
    let script_id = v8::Integer::new(scope, 123);
    let source_map_url = v8::String::new(scope, "").unwrap();
    let resource_is_opaque = v8::Boolean::new(scope, true);
    let is_wasm = v8::Boolean::new(scope, false);
    let is_module = v8::Boolean::new(scope, true);
    v8::ScriptOrigin::new(
        resource_name.into(),
        resource_line_offset,
        resource_column_offset,
        resource_is_shared_cross_origin,
        script_id,
        source_map_url.into(),
        resource_is_opaque,
        is_wasm,
        is_module,
    )
}
