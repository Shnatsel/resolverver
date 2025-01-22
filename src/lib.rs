//!
//!
//! ### Usage
//!
//! ```
//! use cargo_metadata;
//! use resolverver;
//!
//! let metadata = cargo_metadata::MetadataCommand::new().no_deps().exec().unwrap();
//! let toml = std::fs::read_to_string(metadata.workspace_root.join("Cargo.toml")).unwrap();
//! let resolver_version = resolverver::from_toml(&toml).unwrap();
//! println!("Resolver version in this workspace is: {resolver_version:?}");
//! ```

mod error;
mod fields;
mod raw_fields;
mod resolver;

pub use error::Error;
use raw_fields::RawTomlFields;
pub use resolver::Resolver;

pub fn from_toml(workspace_root_cargo_toml: &str) -> Result<Resolver, crate::Error> {
    let parsed: RawTomlFields = toml::from_str(workspace_root_cargo_toml)?;
    Ok(parsed.resolve().resolver()?)
}
