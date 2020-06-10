use rusty_v8 as v8;
mod script_origin;
mod modules;

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
  let origin = script_origin::script_origin(scope, file.clone().as_ref());
  let source = v8::script_compiler::Source::new(code, &origin);
  let mut module = v8::script_compiler::compile_module(scope, source).unwrap();

  let _result = module.instantiate_module(context, modules::resolver);
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
