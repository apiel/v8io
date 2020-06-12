// use rusty_v8 as v8;

// pub fn set_module_loader(
//     scope: v8::FunctionCallbackScope,
//     _args: v8::FunctionCallbackArguments,
//     _rv: v8::ReturnValue,
// ) {
//     println!("set_module_loader");
//     let context = scope.get_current_context().unwrap();

//     let source = v8::String::new(scope, "let yo = 0; function coreModuleLoader() { print(++yo + '<\\n'); return yo; }").unwrap();
//     let mut script = v8::Script::compile(scope, context, source, None).unwrap();
//     script.run(scope, context).unwrap();
// }
