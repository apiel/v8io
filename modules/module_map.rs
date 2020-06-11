use std::collections::HashMap;
use std::sync::Mutex;

pub struct ModuleItem {
    pub absolute_path: String,
}

lazy_static! {
    static ref MODULE_MAP: Mutex<HashMap<i32, ModuleItem>> = Mutex::new(HashMap::new());
}

pub fn insert(identity_hash: i32, module_item: ModuleItem) {
    MODULE_MAP
        .lock()
        .unwrap()
        .insert(identity_hash, module_item);
}
