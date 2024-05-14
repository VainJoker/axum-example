use std::option_env;

use prost_serde::build_with_serde;

fn main() {
    let build_enabled = option_env!("BUILD_PROTO")
        .map(|v| v == "1")
        .unwrap_or(false);

    if !build_enabled {
        println!("=== Skipped compiling protos ===");
        return;
    }

    build_with_serde(include_str!("build_opts.json"));
}
