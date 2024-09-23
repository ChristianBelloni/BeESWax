#[cfg(
    any(
        feature = "google-cloud-policytroubleshooter-iam-v3",
        feature = "google-cloud-policytroubleshooter-iam-v3beta",
    )
)]
pub mod iam;
#[cfg(any(feature = "google-cloud-policytroubleshooter-v1"))]
pub mod v1;
