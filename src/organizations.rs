use clinvoice_schema::Id;
use serde::{Deserialize, Serialize};

/// Configurations for [`Employee`](clinvoice_schema::Employee)s.
///
/// # Examples
///
/// ## TOML
///
/// ```rust
/// # assert!(toml::from_str::<clinvoice_config::Organizations>(r#"
/// employer_id = 1
/// # "#).is_ok());
/// ```
#[derive(
	Copy, Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Organizations
{
	/// The [`Id`] of the [`Organization`](clinvoice_schema::Organization) which uses the CLInvoice
	/// client.
	///
	/// Frontends for CLInvoice should provide mechanisms to assign this setting for the
	/// user.
	pub employer_id: Option<Id>,
}
