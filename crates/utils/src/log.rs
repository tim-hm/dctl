use std::env;

use env_logger::{Builder, Env};

use crate::env::{get_env_name, EnvName};

pub fn init() {
    let level = env::var("RUST_LOG").unwrap_or(String::from("info"));

    let env = Env::default()
        .default_filter_or(level)
        .default_write_style_or("always");

    let mut builder = Builder::from_env(env);

    let exclude_timestamps = get_env_name() != EnvName::Dev;
    if exclude_timestamps {
        builder.format_timestamp(None);
    }

    builder.init();
}
