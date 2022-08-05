use clinvoice_schema::Id;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

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

impl Organizations
{
	/// Returns the `[organizations] employer_id` configuration setting, or an [`Error::NotConfigured`]
	/// if it was not set.
	pub fn employer_id_or_err(&self) -> Result<Id>
	{
		self
			.employer_id
			.ok_or_else(|| Error::NotConfigured("employer_id".into(), "organizations".into()))
	}
}
