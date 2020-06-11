use rusty_v8 as v8;
use std::path::Path;

pub mod compile;
pub mod module_map;
mod custom_module_loader;

pub extern "C" fn dynamic_import_cb(
    context: v8::Local<v8::Context>,
    referrer: v8::Local<v8::ScriptOrModule>,
    specifier: v8::Local<v8::String>,
) -> *mut v8::Promise {
    // core/bindings.rs l.246
    let mut cbs = v8::CallbackScope::new_escapable(context);
    let mut hs = v8::EscapableHandleScope::new(cbs.enter());
    let scope = hs.enter();

    let specifier_str = specifier
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    let referrer_name = referrer.get_resource_name();
    let referrer_str = referrer_name
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    let specifier_path = get_specifier_path(specifier_str, referrer_str);
    // println!("dynamic_import_cb {:?}", specifier_path);

    let resolver = v8::PromiseResolver::new(scope, context).unwrap();
    let promise = resolver.get_promise(scope);

    let mut resolver_handle = v8::Global::new();
    resolver_handle.set(scope, resolver);
    {
        dynamic_resolver(resolver_handle, context, specifier_path, scope);
    }

    &mut *scope.escape(promise)
}

fn dynamic_resolver<'a>(
    mut resolver_handle: v8::Global<v8::PromiseResolver>,
    context: v8::Local<'a, v8::Context>,
    specifier_path: String,
    scope: &mut impl v8::ToLocal<'a>,
) {
    let mut module = compile::compile_file(scope, &specifier_path).unwrap();
    let _result = module.instantiate_module(context, resolver);
    let _result = module.evaluate(scope, context);

    let resolver = resolver_handle.get(scope).unwrap();
    resolver_handle.reset(scope);

    let module_namespace = module.get_module_namespace();
    resolver.resolve(context, module_namespace).unwrap();
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
    let referrer_str = module_map::get_absolute_path(referrer.get_identity_hash());
    let specifier_path = get_specifier_path(specifier_str, referrer_str);

    // println!("specifier_path {:?}", specifier_path);
    let module = compile::compile_file(scope, &specifier_path).unwrap();
    Some(scope.escape(module))
}

fn get_specifier_path<'a>(specifier_str: String, referrer_str: String) -> String {
    Path::new(&referrer_str)
        .parent()
        .unwrap()
        .join(specifier_str)
        .as_path()
        .display()
        .to_string()
}
