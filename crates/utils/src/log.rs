use std::env;

use env_logger::{Builder, Env};

pub fn init() {
    let level = env::var("RUST_LOG").unwrap_or(String::from("info"));

    let env = Env::default()
        .default_filter_or(level)
        .default_write_style_or("always");

    Builder::from_env(env).init();
}
