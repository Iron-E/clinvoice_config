use winvoice_schema::Id;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

/// Configurations for [`Employee`](winvoice_schema::Employee)s.
///
/// # Examples
///
/// ## TOML
///
/// ```rust
/// # assert!(toml::from_str::<winvoice_config::Employees>(r#"
/// id = 1
/// # "#).is_ok());
/// ```
#[derive(
	Copy, Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Employees
{
	/// The [`Id`] of the [`Employee`](winvoice_schema::Employee) which uses this Winvoice
	/// client.
	///
	/// Frontends for Winvoice should provide mechanisms to assign this setting for the user.
	pub id: Option<Id>,
}

impl Employees
{
	/// Returns the `[employees] id` configuration setting, or an [`Error::NotConfigured`] error if
	/// it was not set.
	pub fn id_or_err(&self) -> Result<Id>
	{
		self.id.ok_or_else(|| Error::NotConfigured("id".into(), "employees".into()))
	}
}
