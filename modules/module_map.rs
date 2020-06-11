use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone)]
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

pub fn get(identity_hash: i32) -> Option<ModuleItem> {
    MODULE_MAP.lock().unwrap().get(&identity_hash).cloned()
}

// pub fn get(identity_hash: i32) -> Option<ModuleItem> {
//     MODULE_MAP
//         .lock()
//         .unwrap()
//         .get(&identity_hash)
//         .map(|m| ModuleItem {
//             absolute_path: m.absolute_path.clone(),
//         })
// }
