use rusty_v8 as v8;

pub mod compile;
pub mod custom_module_loader;
pub mod default_loader;
pub mod module;
pub mod module_map;
pub mod plugin_loader;

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

    let module = load_module(scope, context, specifier_str, referrer_str);

    let resolver = v8::PromiseResolver::new(scope, context).unwrap();
    let promise = resolver.get_promise(scope);

    let mut resolver_handle = v8::Global::new();
    resolver_handle.set(scope, resolver);
    {
        dynamic_resolver(resolver_handle, scope, context, module);
    }

    &mut *scope.escape(promise)
}

fn dynamic_resolver<'a>(
    mut resolver_handle: v8::Global<v8::PromiseResolver>,
    scope: &mut impl v8::ToLocal<'a>,
    context: v8::Local<'a, v8::Context>,
    module: module::Module,
) {
    let mut compiled_module = compile::compile_module(scope, module).unwrap();
    let _result = compiled_module.instantiate_module(context, resolver);
    let _result = compiled_module.evaluate(scope, context);

    let resolver = resolver_handle.get(scope).unwrap();
    resolver_handle.reset(scope);

    let module_namespace = compiled_module.get_module_namespace();
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

    let module = load_module_or_plugin(scope, context, specifier_str, referrer_str);
    let compiled_module = compile::compile_module(scope, module).unwrap();
    Some(scope.escape(compiled_module))
}

fn load_module_or_plugin<'sc>(
    scope: &mut impl v8::ToLocal<'sc>,
    context: v8::Local<v8::Context>,
    specifier_str: String,
    referrer_str: String,
) -> module::Module {
    if specifier_str == "core_freeze_plugins" {
        return plugin_loader::freeze_plugins(
            scope,
            context,
            specifier_str.clone(),
            referrer_str.clone(),
        );
    } else if specifier_str.ends_with(".so") || specifier_str.ends_with(".dll") {
        return plugin_loader::load_plugin(
            scope,
            context,
            specifier_str.clone(),
            referrer_str.clone(),
        );
    }
    load_module(scope, context, specifier_str, referrer_str)
}

fn load_module<'sc>(
    scope: &mut impl v8::ToLocal<'sc>,
    context: v8::Local<v8::Context>,
    specifier_str: String,
    referrer_str: String,
) -> module::Module {
    let module_to_load = match custom_module_loader::get_module(
        scope,
        context,
        specifier_str.clone(),
        referrer_str.clone(),
    ) {
        Some(s) => s,
        None => default_loader::get_module(specifier_str, referrer_str),
    };
    let absolute_path = module_to_load.absolute_path;
    let code = match module_to_load.code {
        Some(s) => s,
        None => {
            let err_msg = "Something went wrong reading the file ".to_string() + &absolute_path;
            std::fs::read_to_string(absolute_path.clone()).expect(&err_msg)
        }
    };
    module::Module {
        absolute_path,
        code,
    }
}
