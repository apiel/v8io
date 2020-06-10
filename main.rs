use rusty_v8 as v8;
mod modules;
mod core_functions;

pub fn main() {
  let platform = v8::new_default_platform().unwrap();
  v8::V8::initialize_platform(platform);
  v8::V8::initialize();
  let mut isolate = v8::Isolate::new(Default::default());
  let mut handle_scope = v8::HandleScope::new(&mut isolate);
  let scope = handle_scope.enter();

  // let mut cs = core_functions::init(scope);
  // let scope = cs.enter();

  let object_templ = v8::ObjectTemplate::new(scope);
  let function_templ = v8::FunctionTemplate::new(scope, fortytwo_callback);
  let name = v8::String::new(scope, "yo").unwrap();
  object_templ.set(name.into(), function_templ.into());
  let function_templ = v8::FunctionTemplate::new(scope, print);
  let name = v8::String::new(scope, "print").unwrap();
  object_templ.set(name.into(), function_templ.into());
  let context = v8::Context::new_from_template(scope, object_templ);
  let mut cs = v8::ContextScope::new(scope, context);
  let scope = cs.enter();


  let file = get_bootstrap_file();
  let mut module = modules::compile_file(scope, file.clone().as_ref()).unwrap();

  let _result = module.instantiate_module(context, modules::resolver);
  let _result = module.evaluate(scope, context);
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
