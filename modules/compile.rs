use rusty_v8 as v8;

use crate::modules::module;
use crate::modules::module_map;

pub fn compile_file<'sc>(
    scope: &mut impl v8::ToLocal<'sc>,
    file: &str,
) -> Option<v8::Local<'sc, v8::Module>> {
    let err_msg = "Something went wrong reading the file ".to_string() + file;
    let code = std::fs::read_to_string(file.clone()).expect(&err_msg);

    compile_module(scope, module::Module {
        absolute_path: file.to_string(),
        code,
    })
}

pub fn compile_module<'sc>(
    scope: &mut impl v8::ToLocal<'sc>,
    module: module::Module,
) -> Option<v8::Local<'sc, v8::Module>> {
    let source_string = v8::String::new(scope, &module.code).unwrap();
    let compiled_module = compile(scope, &module.absolute_path, source_string);

    module_map::insert(compiled_module.unwrap().get_identity_hash(), module);

    compiled_module
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
