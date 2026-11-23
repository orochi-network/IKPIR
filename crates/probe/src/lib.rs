//! Build-time metadata provider for the IKPIR workspace.

use std::env;

/// Returns the build environment metadata string.
pub fn build_metadata() -> String {
    format!(
        "ikpir-probe {} (rustc {}, {} CI)",
        env!("CARGO_PKG_VERSION"),
        option_env!("RUSTC_VERSION").unwrap_or("unknown"),
        if env::var("CI").map(|v| v == "true").unwrap_or(false) {
            "in"
        } else {
            "no"
        }
    )
}
