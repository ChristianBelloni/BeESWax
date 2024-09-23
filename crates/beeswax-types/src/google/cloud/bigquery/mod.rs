#[cfg(any(feature = "google-cloud-bigquery-analyticshub-v1"))]
pub mod analyticshub;
#[cfg(
    any(
        feature = "google-cloud-bigquery-biglake-v1",
        feature = "google-cloud-bigquery-biglake-v1alpha1",
    )
)]
pub mod biglake;
#[cfg(
    any(
        feature = "google-cloud-bigquery-connection-v1",
        feature = "google-cloud-bigquery-connection-v1beta1",
    )
)]
pub mod connection;
#[cfg(any(feature = "google-cloud-bigquery-dataexchange-v1beta1"))]
pub mod dataexchange;
#[cfg(
    any(
        feature = "google-cloud-bigquery-datapolicies-v1",
        feature = "google-cloud-bigquery-datapolicies-v1beta1",
    )
)]
pub mod datapolicies;
#[cfg(any(feature = "google-cloud-bigquery-datatransfer-v1"))]
pub mod datatransfer;
#[cfg(any(feature = "google-cloud-bigquery-logging-v1"))]
pub mod logging;
#[cfg(
    any(
        feature = "google-cloud-bigquery-migration-v2",
        feature = "google-cloud-bigquery-migration-v2alpha",
    )
)]
pub mod migration;
#[cfg(any(feature = "google-cloud-bigquery-reservation-v1"))]
pub mod reservation;
#[cfg(
    any(
        feature = "google-cloud-bigquery-storage-v1",
        feature = "google-cloud-bigquery-storage-v1alpha",
        feature = "google-cloud-bigquery-storage-v1beta1",
        feature = "google-cloud-bigquery-storage-v1beta2",
    )
)]
pub mod storage;
#[cfg(any(feature = "google-cloud-bigquery-v2"))]
pub mod v2;
