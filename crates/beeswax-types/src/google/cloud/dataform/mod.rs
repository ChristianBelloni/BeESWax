#[cfg(any(feature = "google-cloud-dataform-logging-v1"))]
pub mod logging;
#[cfg(any(feature = "google-cloud-dataform-v1alpha2"))]
pub mod v1alpha2;
#[cfg(any(feature = "google-cloud-dataform-v1beta1"))]
pub mod v1beta1;
