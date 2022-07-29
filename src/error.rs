#![allow(clippy::std_instead_of_core)]

use std::io;

use thiserror::Error;

use crate::Adapters;

/// An [`Error`](std::error::Error) for the crate.
#[derive(Debug, Error)]
pub enum Error
{
	#[allow(missing_docs)]
	#[error("Using this adapter requires the `{0}` feature")]
	FeatureNotFound(Adapters),

	#[allow(missing_docs)]
	#[error(transparent)]
	Io(#[from] io::Error),

	#[allow(missing_docs)]
	#[error("The `{0}` key in the `[{1}]` field of the configuration file has no value")]
	NotConfigured(String, String),

	#[allow(missing_docs)]
	#[error(transparent)]
	TomlDe(#[from] toml::de::Error),

	#[allow(missing_docs)]
	#[error(transparent)]
	TomlSer(#[from] toml::ser::Error),
}

clinvoice_error::AliasResult!();
