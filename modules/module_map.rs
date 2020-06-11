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

// pub fn get(identity_hash: i32) -> Option<&'static ModuleItem> {
//     MODULE_MAP.lock().unwrap().get(&identity_hash).clone() //.cloned() //.clone() //.unwrap()
// }

// pub fn get<'a>(identity_hash: &'a i32) -> Option<&'a ModuleItem> {
// pub fn get(identity_hash: i32) -> Option<&'static ModuleItem> {
// pub fn get(identity_hash: &'static i32) -> Option<&'static ModuleItem> {
//     // pub fn get(identity_hash: i32) -> Option<ModuleItem> {
//     MODULE_MAP.lock().unwrap().get(identity_hash).clone() //.cloned() //.clone() //.unwrap()
// }

// // pub fn get_absolute_path(identity_hash: i32) -> std::string::String {
// pub fn get_absolute_path(identity_hash: i32) -> &'static str {
//     let module_item = MODULE_MAP.lock().unwrap().get(&identity_hash);
//     match module_item {
//         Some(item) => return item.absolute_path.to_string().as_ref(),
//         None => return "",
//     };
// }

// pub fn get_absolute_path(identity_hash: i32) -> std::string::String {
//     let module_item = MODULE_MAP.lock().unwrap().get(&identity_hash);
//     match module_item {
//         Some(item) => return item.absolute_path.to_string(),
//         None => return "".to_string(),
//     };
// }
