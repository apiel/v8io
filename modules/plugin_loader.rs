use crate::modules::default_loader;
use crate::modules::module;
use rusty_v8 as v8;
use std::collections::HashMap;
use std::sync::Mutex;

extern crate libloading as lib;

type RunFunc = unsafe fn(&str) -> Option<String>;
type GetNameFunc = unsafe fn() -> String;
type GetCodeFunc = unsafe fn() -> String;

lazy_static! {
    static ref PLUGIN_MAP: Mutex<HashMap<String, lib::Library>> = Mutex::new(HashMap::new());
}

pub fn insert(name: String, plugin: lib::Library) {
    PLUGIN_MAP.lock().unwrap().insert(name, plugin);
}

pub fn instantiate(name: String, params_str: String) -> Option<String> {
    let plugin_map = PLUGIN_MAP.lock().unwrap();
    let plugin = plugin_map.get(&name);
    if let Some(item) = plugin {
        unsafe {
            let run: lib::Symbol<RunFunc> = item.get(b"run").unwrap();
            return run(params_str.as_ref());
        }
    }
    None
}

pub fn instantiate_async<'a>(
    name: String,
    params_str: String,
    scope: &mut impl v8::ToLocal<'a>,
    context: v8::Local<'a, v8::Context>,
    mut resolver_handle: v8::Global<v8::PromiseResolver>,
) {
    let resolver = resolver_handle.get(scope).unwrap();
    resolver_handle.reset(scope);

    let plugin_map = PLUGIN_MAP.lock().unwrap();
    let plugin = plugin_map.get(&name);
    if let Some(item) = plugin {
        unsafe {
            let run: lib::Symbol<RunFunc> = item.get(b"run").unwrap();
            let response = run(params_str.as_ref());

            if let Some(res) = response {
                let value = v8::String::new(scope, &res).unwrap();
                resolver.resolve(context, value.into()).unwrap();
                return;
            }
        }
    }
    resolver
        .resolve(context, v8::undefined(scope).into())
        .unwrap();
}

pub fn load_plugin<'sc>(
    _scope: &mut impl v8::ToLocal<'sc>,
    _context: v8::Local<v8::Context>,
    specifier_str: String,
    referrer_str: String,
) -> module::Module {
    let absolute_path = default_loader::get_module(specifier_str, referrer_str).absolute_path;

    let plugin = lib::Library::new(absolute_path.clone()).unwrap();

    unsafe {
        let get_name: lib::Symbol<GetNameFunc> = plugin.get(b"get_name").unwrap();
        let name = get_name();

        let get_code: lib::Symbol<GetCodeFunc> = plugin.get(b"get_code").unwrap();
        let code = get_code();

        insert(name, plugin);

        module::Module {
            absolute_path,
            code,
        }
    }
}
