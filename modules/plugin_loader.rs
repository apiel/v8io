use crate::modules::default_loader;
use crate::modules::module;
use rusty_v8 as v8;
use std::collections::HashMap;
use std::sync::Mutex;

extern crate libloading as lib;

type RunFunc = unsafe fn(isize, isize) -> isize;
type GetNameFunc = unsafe fn() -> String;
type GetCodeFunc = unsafe fn() -> String;

pub struct PluginItem {
    pub lib: lib::Library,
}

lazy_static! {
    static ref PLUGIN_MAP: Mutex<HashMap<String, PluginItem>> = Mutex::new(HashMap::new());
}

pub fn insert(name: String, plugin_item: PluginItem) {
    PLUGIN_MAP.lock().unwrap().insert(name, plugin_item);
}

// pub fn get(name: String) -> Option<PluginItem> {
//     // PLUGIN_MAP.lock().unwrap().get(&name).cloned()
// }

pub fn instanciate(name: String) {
    // // get(name).unwrap().run(1, 2);
    // let plugin_item = PLUGIN_MAP.lock().unwrap().get(&name);
    // if let Some(item) = plugin_item {
    //     let lib = item.lib;
    // }
}

pub fn load_plugin<'sc>(
    _scope: &mut impl v8::ToLocal<'sc>,
    _context: v8::Local<v8::Context>,
    specifier_str: String,
    referrer_str: String,
) -> module::Module {
    let absolute_path = default_loader::get_module(specifier_str, referrer_str).absolute_path;

    let lib = lib::Library::new(absolute_path.clone()).unwrap();
    let plugin_item = PluginItem { lib };

    unsafe {
        let get_name: lib::Symbol<GetNameFunc> = plugin_item.lib.get(b"get_name").unwrap();
        let name = get_name();

        let get_code: lib::Symbol<GetCodeFunc> = plugin_item.lib.get(b"get_code").unwrap();
        let code = get_code();
        // let run: lib::Symbol<RunFunc> = lib.get(b"run").unwrap();
        // let answer = run(1, 2);
        // println!("({:?}) 1 + 2 = {}", name, answer);

        insert(name, plugin_item);
        module::Module {
            absolute_path,
            code,
        }
    }
}
