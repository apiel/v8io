use std::collections::HashMap;

struct Module {
    absolute_path: String,
}

struct ModuleMap {
    list: HashMap<i32, Module>,
}

impl ModuleMap {
    pub fn new() -> Self {
        ModuleMap {
            list: HashMap::new(),
        }
    }

    pub fn insert(&mut self, identity_hash: i32, module: Module) {
        self.list.insert(identity_hash, module);
    }
}
