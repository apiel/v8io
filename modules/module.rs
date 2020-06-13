pub struct ModuleToLoad {
    pub absolute_path: String,
    pub code: Option<String>,
}

// #[derive(Clone)]
pub struct Module {
    pub absolute_path: String,
    pub code: String,
}
