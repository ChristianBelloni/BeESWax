#[cfg(any(feature = "google-shopping-css-v1"))]
pub mod css;
#[cfg(
    any(
        feature = "google-shopping-merchant-accounts-v1beta",
        feature = "google-shopping-merchant-conversions-v1beta",
        feature = "google-shopping-merchant-datasources-v1beta",
        feature = "google-shopping-merchant-inventories-v1beta",
        feature = "google-shopping-merchant-lfp-v1beta",
        feature = "google-shopping-merchant-notifications-v1beta",
        feature = "google-shopping-merchant-products-v1beta",
        feature = "google-shopping-merchant-promotions-v1beta",
        feature = "google-shopping-merchant-quota-v1beta",
        feature = "google-shopping-merchant-reports-v1beta",
    )
)]
pub mod merchant;
#[cfg(any(feature = "google-shopping-type"))]
pub mod r#type;
