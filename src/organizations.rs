use serde::{Deserialize, Serialize};
use winvoice_schema::Id;

use crate::{Error, Result};

/// Configurations for [`Employee`](winvoice_schema::Employee)s.
///
/// # Examples
///
/// ## TOML
///
/// ```rust
/// # assert!(toml::from_str::<winvoice_config::Organizations>(r#"
/// employer_id = "e58fbcdc-97cf-4bbf-8d11-a51c0380091d"
/// # "#).is_ok());
/// ```
#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Organizations
{
	/// The [`Id`] of the [`Organization`](winvoice_schema::Organization) which uses the Winvoice
	/// client.
	///
	/// Frontends for Winvoice should provide mechanisms to assign this setting for the
	/// user.
	pub employer_id: Option<Id>,
}

impl Organizations
{
	/// Returns the `[organizations] employer_id` configuration setting, or an
	/// [`Error::NotConfigured`] if it was not set.
	pub fn employer_id_or_err(&self) -> Result<Id>
	{
		self.employer_id.ok_or_else(|| Error::NotConfigured("employer_id".into(), "organizations".into()))
	}
}
