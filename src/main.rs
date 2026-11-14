#![forbid(unsafe_code)]

#[path = "../generated/rust/env.rs"]
mod env;
#[path = "../generated/rust/runtime.rs"]
mod env_runtime;

use ores_otel_sidecar::{runtime, SidecarConfig, SidecarIdentity};

fn main() {
    let values = env_runtime::load_from_os();
    // An invalid explicitly configured bind must not silently start a sidecar
    // on a fallback address: that hides deployment/configuration errors.
    let cfg = SidecarConfig::from_bind(
        SidecarIdentity::new(env::SERVICE, env::BIND),
        &values.bind,
        false,
    )
    .unwrap_or_else(|err| {
        eprintln!("invalid {} value {:?}: {err:?}", env::BIND, values.bind);
        std::process::exit(2);
    });
    runtime::run(&cfg);
}
