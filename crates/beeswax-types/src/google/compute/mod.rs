#[cfg(
    any(
        feature = "google-compute-logging-dr-v1",
        feature = "google-compute-logging-gdnsusage-v1",
    )
)]
pub mod logging;
