mod default;

use serde::{Deserialize, Serialize};
use winvoice_schema::Currency;

/// Configurations for [`Invoice`](winvoice_schema::Invoice)s.
///
/// # Examples
///
/// ## TOML
///
/// ```rust
/// # assert!(toml::from_str::<winvoice_config::Invoices>(r#"
/// default_currency = "USD"
/// # "#).is_ok());
/// ```
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Invoices
{
	/// The default currency should be used for the `hourly_rate` of an
	/// [`Invoice`](winvoice_schema::Invoice).
	pub default_currency: Currency,
}
