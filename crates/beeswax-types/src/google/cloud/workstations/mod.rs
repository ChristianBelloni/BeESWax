#[cfg(any(feature = "google-cloud-workstations-logging-v1"))]
pub mod logging;
#[cfg(any(feature = "google-cloud-workstations-v1"))]
pub mod v1;
#[cfg(any(feature = "google-cloud-workstations-v1beta"))]
pub mod v1beta;
