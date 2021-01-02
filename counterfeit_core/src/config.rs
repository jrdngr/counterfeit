#[derive(Debug, Clone)]
pub struct CounterfeitConfig {
    pub base_path: String,
    pub write: bool,
    pub create_missing: bool,
    pub silent: bool,
    pub prefix: String,
    pub postfix: String,
}
