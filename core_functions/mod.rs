use rusty_v8 as v8;

mod print;
mod plugin;
mod args;
// mod set_module_loader;

pub fn init<'sc>(
    scope: &'sc mut impl v8::ToLocal<'sc>,
) -> (
    v8::Local<'_, v8::ObjectTemplate>,
    &'sc mut impl v8::ToLocal<'sc>,
) {
    let object_templ = v8::ObjectTemplate::new(scope);

    let function_templ = v8::FunctionTemplate::new(scope, core_test_callback);
    let name = v8::String::new(scope, "coreTest").unwrap();
    object_templ.set(name.into(), function_templ.into());

    let function_templ = v8::FunctionTemplate::new(scope, args::args);
    let name = v8::String::new(scope, "args").unwrap();
    object_templ.set(name.into(), function_templ.into());

    let function_templ = v8::FunctionTemplate::new(scope, print::print);
    let name = v8::String::new(scope, "print").unwrap();
    object_templ.set(name.into(), function_templ.into());

    let function_templ = v8::FunctionTemplate::new(scope, print::eprint);
    let name = v8::String::new(scope, "eprint").unwrap();
    object_templ.set(name.into(), function_templ.into());

    let function_templ = v8::FunctionTemplate::new(scope, plugin::core_instantiate);
    let name = v8::String::new(scope, "coreInstantiate").unwrap();
    object_templ.set(name.into(), function_templ.into());

    let function_templ = v8::FunctionTemplate::new(scope, plugin::core_instantiate_async);
    let name = v8::String::new(scope, "coreInstantiateAsync").unwrap();
    object_templ.set(name.into(), function_templ.into());

    // let function_templ = v8::FunctionTemplate::new(scope, set_module_loader::set_module_loader);
    // let name = v8::String::new(scope, "setModuleLoader").unwrap();
    // object_templ.set(name.into(), function_templ.into());

    (object_templ, scope)
}

fn core_test_callback(
    scope: v8::FunctionCallbackScope,
    _: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let context = scope.get_current_context().unwrap();
    let resolver = v8::PromiseResolver::new(scope, context).unwrap();
    let promise = resolver.get_promise(scope);

    let mut resolver_handle = v8::Global::new();
    resolver_handle.set(scope, resolver);
    {
        // let resolver = resolver_handle.get(scope).unwrap();
        resolver_handle.reset(scope);

        let value = v8::String::new(scope, "test").unwrap();
        resolver.resolve(context, value.into()).unwrap();
    }

    rv.set(promise.into());

    // rv.set(v8::Integer::new(scope, 42).into());
}
