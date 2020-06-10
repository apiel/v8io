use rusty_v8 as v8;

pub fn main() {
  let platform = v8::new_default_platform().unwrap();
  v8::V8::initialize_platform(platform);
  v8::V8::initialize();
  let mut isolate = v8::Isolate::new(Default::default());
  let mut handle_scope = v8::HandleScope::new(&mut isolate);
  let scope = handle_scope.enter();
  let context = v8::Context::new(scope);
  let mut context_scope = v8::ContextScope::new(scope, context);
  let scope = context_scope.enter();

  let file = get_bootstrap_file();
  let contents = std::fs::read_to_string(file.clone()).expect("Something went wrong reading the file");
  let code = v8::String::new(scope, &contents).unwrap();
  let origin = script_origin(scope, file.clone().as_ref());
  let source = v8::script_compiler::Source::new(code, &origin);
  let mut module = v8::script_compiler::compile_module(scope, source).unwrap();

  let _result = module.instantiate_module(context, compile_specifier_as_module_resolve_callback);
  // let _result = module.evaluate(scope, context);
}

fn get_bootstrap_file() -> std::string::String {
  match std::env::var("V8IO_BOOSTRAP") {
    Ok(val) => return val,
    Err(_) => {
      let file = std::env::current_exe().unwrap().parent().unwrap().join("bootstrap.js");
      return file.into_os_string().into_string().unwrap();
    }
  };
}

fn compile_specifier_as_module_resolve_callback<'a>(
  context: v8::Local<'a, v8::Context>,
  specifier: v8::Local<'a, v8::String>,
  _referrer: v8::Local<'a, v8::Module>,
) -> Option<v8::Local<'a, v8::Module>> {
  let mut cbs = v8::CallbackScope::new_escapable(context);
  let mut hs = v8::EscapableHandleScope::new(cbs.enter());
  let scope = hs.enter();

  let specifier_str = specifier.to_rust_string_lossy(scope);
  println!("specifier_str {:?}", specifier_str);

  let origin = script_origin(scope, "module.js");
  let source = v8::script_compiler::Source::new(specifier, &origin);
  let module = v8::script_compiler::compile_module(scope, source).unwrap();
  Some(scope.escape(module))
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
