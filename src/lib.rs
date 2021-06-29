use ckb_tool::ckb_types::bytes::Bytes;
use std::env;
use std::fs;
use std::path::PathBuf;

#[cfg(test)]
mod tests;
mod protocol;

pub struct Loader(PathBuf);

impl Default for Loader {
    fn default() -> Self {
        Self::with_test_env()
    }
}

impl Loader {
    fn with_test_env() -> Self {
        let dir = env::current_dir().unwrap();
        let mut base_path = PathBuf::new();
        base_path.push(dir);
        base_path.push("c");
        base_path.push("build");
        Loader(base_path)
    }

    pub fn load_binary(&self, name: &str) -> Bytes {
        let mut path = self.0.clone();
        path.push(name);
        fs::read(path).expect("binary").into()
    }
}
