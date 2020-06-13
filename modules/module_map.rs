use std::collections::HashMap;
use std::sync::Mutex;
use crate::modules::module;

#[derive(Clone)]
pub struct ModuleItem {
    pub absolute_path: String,
}

lazy_static! {
    static ref MODULE_MAP: Mutex<HashMap<i32, ModuleItem>> = Mutex::new(HashMap::new());
}

pub fn insert(identity_hash: i32, module: module::Module) {
    // should we insert the whole module?
    // but then, it would use memory for nothing
    let module_item = ModuleItem {
        absolute_path: module.absolute_path
    };
    MODULE_MAP
        .lock()
        .unwrap()
        .insert(identity_hash, module_item);
}

pub fn get(identity_hash: i32) -> Option<ModuleItem> {
    MODULE_MAP.lock().unwrap().get(&identity_hash).cloned()
}

pub fn get_absolute_path(identity_hash: i32) -> std::string::String {
    get(identity_hash).unwrap().absolute_path
}
