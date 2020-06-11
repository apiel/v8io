// use std::sync::Mutex;

// instead to evaluate a full new isolated instance
// i could just call look if a specific function
// exist in the current instance and execute it.
//
// something like getGlobal->coreModuleLoader
// or
// let result = eval(scope, context, "Object.expando").unwrap();
// assert!(result.is_number());
// let expected = v8::Number::new(scope, 10.);
// assert!(result.strict_equals(expected.into()));
//
// we still might need some global settings...


// // #[derive(Clone)]
// pub struct CustomModuleLoader {
//     pub active: bool,
//     // code
//     // state (in json)
// }

// lazy_static! {
//     static ref MODULE_LOADER: Mutex<CustomModuleLoader> = Mutex::new(CustomModuleLoader {
//         active: false
//     });
// }

// pub fn set() {
//     MODULE_LOADER.lock().unwrap().active = true;
// }
