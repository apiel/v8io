use rusty_v8 as v8;

mod core_functions;
mod modules;

#[macro_use(lazy_static)]
extern crate lazy_static;

pub fn main() {
  let platform = v8::new_default_platform().unwrap();
  v8::V8::initialize_platform(platform);
  v8::V8::initialize();
  let mut isolate = v8::Isolate::new(Default::default());
  isolate.set_host_import_module_dynamically_callback(modules::dynamic_import_cb);
  let mut handle_scope = v8::HandleScope::new(&mut isolate);
  let scope = handle_scope.enter();

  let (object_templ, scope) = core_functions::init(scope);
  let context = v8::Context::new_from_template(scope, object_templ);
  let mut cs = v8::ContextScope::new(scope, context);
  let scope = cs.enter();

// let source = v8::String::new(scope, "function coreModuleLoader() {}").unwrap();
// let mut script = v8::Script::compile(scope, context, source, None).unwrap();
// let result = script.run(scope, context).unwrap();


  let file = get_bootstrap_file();
  let mut module = modules::compile::compile_file(scope, file.clone().as_ref()).unwrap();

  let _result = module.instantiate_module(context, modules::resolver);
  let _result = module.evaluate(scope, context);

  // // for testing
  // let result = modules::eval(scope, context, "typeof coreModuleLoader === 'function' && coreModuleLoader()").unwrap();
  // let result = result.to_string(scope).unwrap();
  // println!("coreModuleLoader: {}", result.to_rust_string_lossy(scope));
  // let result = modules::eval(scope, context, "typeof coreYo").unwrap();
  // let result = result.to_string(scope).unwrap();
  // println!("coreYo: {}", result.to_rust_string_lossy(scope));

  // let global = v8::Global::new_from(scope, local);
  // maybe create_message_argument_lifetimes in test_api
  // context.global(scope);

  // let item_name = v8::String::new(scope, "coreYo").unwrap();
  // let res = context.global(scope).get(scope, context, item_name.into());
  // let item_name = v8::String::new(scope, "coreModuleLoader").unwrap();
  // let res = context.global(scope).get(scope, context, item_name.into());
  // assert!(res.into().is_function());
  // res.into();

  // let source = v8::String::new(scope, "function coreModuleLoader() {}").unwrap();
  // let mut script = v8::Script::compile(scope, context, source, None).unwrap();
  // let result = script.run(scope, context).unwrap();

  // let source = v8::String::new(scope, "typeof coreModuleLoader").unwrap();
  // let mut script = v8::Script::compile(scope, context, source, None).unwrap();
  // let result = script.run(scope, context).unwrap();
  // let result = result.to_string(scope).unwrap();
  // println!("res: {}", result.to_rust_string_lossy(scope));

}

fn get_bootstrap_file() -> std::string::String {
  match std::env::var("V8IO_BOOSTRAP") {
    Ok(val) => return val,
    Err(_) => {
      let file = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("bootstrap.js");
      return file.into_os_string().into_string().unwrap();
    }
  };
}
