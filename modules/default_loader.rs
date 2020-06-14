use crate::modules::module;
use std::path::Path;

pub fn get_module(specifier_str: String, referrer_str: String) -> module::ModuleToLoad {
    module::ModuleToLoad {
        absolute_path: Path::new(&referrer_str)
            .parent()
            .unwrap()
            .join(specifier_str)
            .as_path()
            .display()
            .to_string(),
        code: None,
    }
}
