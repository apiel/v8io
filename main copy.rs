use rusty_v8 as v8;

// let _setup_guard = setup();
// let mut isolate = v8::Isolate::new(Default::default());
// isolate.set_promise_reject_callback(promise_reject_callback);
// {
//   let mut hs = v8::HandleScope::new(&mut isolate);
//   let scope = hs.enter();
//   let context = v8::Context::new(scope);
//   let mut cs = v8::ContextScope::new(scope, context);
//   let scope = cs.enter();

//   let source = "1+2";
//   let script_origin = mock_script_origin(scope, "foo.js");
//   let source =
//     v8::script_compiler::Source::new(v8_str(scope, source), &script_origin);

//   let result = v8::script_compiler::compile_module(scope, source);
//   assert!(result.is_some());
// }

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

  // fn yo
  let object_templ = v8::ObjectTemplate::new(scope);
  let function_templ = v8::FunctionTemplate::new(scope, fortytwo_callback);
  let name = v8_str(scope, "yo");
  object_templ.set(name.into(), function_templ.into());
  // let context = v8::Context::new_from_template(scope, object_templ);
  // let mut cs = v8::ContextScope::new(scope, context);
  // let scope = cs.enter();

  // let function_templ = v8::FunctionTemplate::new(scope, plugin);
  // let name = v8_str(scope, "plugin");
  // object_templ.set(name.into(), function_templ.into());

  // fn fn_callback2
  // let object_templ = v8::ObjectTemplate::new(scope);
  let function_templ = v8::FunctionTemplate::new(scope, print);
  let name = v8_str(scope, "print");
  object_templ.set(name.into(), function_templ.into());
  let context = v8::Context::new_from_template(scope, object_templ);
  let mut cs = v8::ContextScope::new(scope, context);
  let scope = cs.enter();

  let contents =
    std::fs::read_to_string("./demo/main.js").expect("Something went wrong reading the file");
  let code = v8::String::new(scope, &contents).unwrap();

  let origin = mock_script_origin(scope, "./demo/main.js");
  let source = v8::script_compiler::Source::new(code, &origin);
  let mut module = v8::script_compiler::compile_module(scope, source).unwrap();
  assert_eq!(v8::ModuleStatus::Uninstantiated, module.get_status());

  let result = module.instantiate_module(context, compile_specifier_as_module_resolve_callback);
  assert!(result.unwrap());
  assert_eq!(v8::ModuleStatus::Instantiated, module.get_status());

  let _result = module.evaluate(scope, context);

  let result = eval(scope, context, "Object.expando").unwrap();

  let result = result.to_string(scope).unwrap();
  println!("result: {}", result.to_rust_string_lossy(scope));
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

  let origin = mock_script_origin(scope, "module.js");
  let source = v8::script_compiler::Source::new(specifier, &origin);
  let module = v8::script_compiler::compile_module(scope, source).unwrap();
  Some(scope.escape(module))
}

fn mock_script_origin<'sc>(
  scope: &mut impl v8::ToLocal<'sc>,
  resource_name_: &str,
) -> v8::ScriptOrigin<'sc> {
  let resource_name = v8_str(scope, resource_name_);
  let resource_line_offset = v8::Integer::new(scope, 0);
  let resource_column_offset = v8::Integer::new(scope, 0);
  let resource_is_shared_cross_origin = v8::Boolean::new(scope, true);
  let script_id = v8::Integer::new(scope, 123);
  let source_map_url = v8_str(scope, "source_map_url");
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

fn v8_str<'sc>(scope: &mut impl v8::ToLocal<'sc>, s: &str) -> v8::Local<'sc, v8::String> {
  v8::String::new(scope, s).unwrap()
}

fn eval<'sc>(
  scope: &mut impl v8::ToLocal<'sc>,
  context: v8::Local<v8::Context>,
  code: &str,
) -> Option<v8::Local<'sc, v8::Value>> {
  let mut hs = v8::EscapableHandleScope::new(scope);
  let scope = hs.enter();
  let source = v8_str(scope, code);
  let mut script = v8::Script::compile(scope, context, source, None).unwrap();
  let r = script.run(scope, context);
  r.map(|v| scope.escape(v))
}

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

// fn plugin(
//   scope: v8::FunctionCallbackScope,
//   _: v8::FunctionCallbackArguments,
//   mut _rv: v8::ReturnValue,
// ) {
//   println!("in plugin");
//   let context = v8::Context::get_current_context();
//   // let context = v8::Context::get;
//   let result = eval(scope, context, "Object.expando").unwrap();
//   let result = result.to_string(scope).unwrap();
//   println!("in plugin: {}", result.to_rust_string_lossy(scope));
//   // rv.set(v8::Integer::new(scope, 42).into());
// }

// fn plugin(my_context: v8::Local<v8::Context>) {
//   return |scope: v8::FunctionCallbackScope,
//     _: v8::FunctionCallbackArguments,
//     mut _rv: v8::ReturnValue| {
//     println!("in plugin");
//     let result = eval(scope, my_context, "Object.expando").unwrap();
//     // let result = result.to_string(scope).unwrap();
//     // println!("in plugin: {}", result.to_rust_string_lossy(scope));
//     // rv.set(v8::Integer::new(scope, 42).into());
//   };
// }

// let plugin = |my_context: v8::Local<v8::Context>|
// |scope: v8::FunctionCallbackScope,
//  _: v8::FunctionCallbackArguments,
//  mut _rv: v8::ReturnValue| {
//   println!("in plugin");
//   let result = eval(scope, my_context, "Object.expando").unwrap();
//   // let result = result.to_string(scope).unwrap();
//   // println!("in plugin: {}", result.to_rust_string_lossy(scope));
//   // rv.set(v8::Integer::new(scope, 42).into());
// };
