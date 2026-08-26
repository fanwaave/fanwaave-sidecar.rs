#![forbid(unsafe_code)]

use fanwaave_sidecar::{config::SidecarConfig, runtime};

fn main() {
    let cfg = SidecarConfig::from_env();
    runtime::run(&cfg);
}

