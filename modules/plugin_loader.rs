use crate::modules::default_loader;
use crate::modules::module;
use rusty_v8 as v8;
use std::collections::HashMap;
use std::sync::Mutex;

extern crate libloading as lib;

type RunFunc = unsafe fn(isize, isize) -> isize;
type GetNameFunc = unsafe fn() -> String;
type GetCodeFunc = unsafe fn() -> String;

// pub struct PluginItem {
//     pub lib: lib::Library,
// }

lazy_static! {
    static ref PLUGIN_MAP: Mutex<HashMap<String, lib::Library>> = Mutex::new(HashMap::new());
}

// lazy_static! {
//     static ref PLUGIN_MAP2: HashMap<String, PluginItem> = {
//         let mut m = HashMap::new();
//         let lib =
//             lib::Library::new("/home/alex/dev/deno/pkg/v8io/plugins/demo/libadder.so").unwrap();
//         m.insert("yo".to_string(), PluginItem { lib });
//         m
//     };
// }
// pub fn instanciate2(name: String) {
//     // // get(name).unwrap().run(1, 2);
//     let plugin_item = PLUGIN_MAP2.get(&name);
//     if let Some(item) = plugin_item {
//         unsafe {
//             let run: lib::Symbol<RunFunc> = item.lib.get(b"run").unwrap();
//             let answer = run(1, 2);
//             println!("({:?}) 1 + 2 = {}", name, answer);
//         }
//     }
// }

pub fn insert(name: String, plugin: lib::Library) {
    PLUGIN_MAP.lock().unwrap().insert(name, plugin);
}

// pub fn get(name: String) -> Option<PluginItem> {
//     // PLUGIN_MAP.lock().unwrap().get(&name).cloned()
// }

pub fn instanciate(name: String) {
    let plugin_map = PLUGIN_MAP.lock().unwrap();
    let plugin = plugin_map.get(&name);
    if let Some(item) = plugin {
        unsafe {
            let run: lib::Symbol<RunFunc> = item.get(b"run").unwrap();
            let answer = run(1, 2);
            println!("({:?}) 1 + 2 = {}", name, answer);
        }
    }
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
        // let run: lib::Symbol<RunFunc> = lib.get(b"run").unwrap();
        // let answer = run(1, 2);
        // println!("({:?}) 1 + 2 = {}", name, answer);

        insert(name, plugin);
        module::Module {
            absolute_path,
            code,
        }
    }
}
