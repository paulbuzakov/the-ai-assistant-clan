/// Re-export the standard log macros so callers only need to depend on `shared`.
pub use log::{debug, error, info, trace, warn};

use std::sync::OnceLock;

static INIT: OnceLock<()> = OnceLock::new();

/// Initialise the global logger.
///
/// Safe to call multiple times — subsequent calls are no-ops.
/// The log level is controlled by the `RUST_LOG` environment variable
/// (defaults to `info` when the variable is unset).
pub fn init() {
    INIT.get_or_init(|| {
        env_logger::Builder::from_env(
            env_logger::Env::default().default_filter_or("info"),
        )
        .init();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_is_idempotent() {
        // Calling init twice must not panic.
        init();
        init();
    }
}
