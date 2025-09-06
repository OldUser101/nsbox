pub mod config;

pub use config::{Config, NamespacesConfig};

pub struct Sandbox {
    config: Config,
}

impl Sandbox {
    pub fn new(config: Option<Config>) -> Self {
        Self {
            config: config.unwrap_or_default(),
        }
    }

    pub fn run<F>(&mut self, mut f: F) -> Result<i32, String>
    where
        F: FnMut() -> isize + 'static,
    {
        unimplemented!();
    }
}
