use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
struct Package {
    resolver: Option<String>,
    edition: Option<Edition>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(untagged)]
enum Edition {
    Edition(String),
    InheritWorkspace { workspace: bool },
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
struct Workspace {
    package: Option<Package>,
    resolver: Option<String>,
}

/// Raw deserialized TOML fields, before resolving workspace inheritance
#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
struct RawTomlFields {
    package: Option<Package>,
    workspace: Option<Workspace>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_the_fields() {
        let toml = "
[package]
name = \"sample-package\"
version = \"0.1.0\"
edition.workspace = true
resolver = \"1\"

[dependencies]

[workspace]
package.edition = \"2021\"
resolver = \"2\"
";

        let expected = RawTomlFields {
            package: Some(Package {
                resolver: Some("1".to_owned()),
                edition: Some(Edition::InheritWorkspace { workspace: true }),
            }),
            workspace: Some(Workspace {
                package: Some(Package {
                    resolver: None,
                    edition: Some(Edition::Edition("2021".to_owned())),
                }),
                resolver: Some("2".to_owned()),
            }),
        };

        let parsed: RawTomlFields = toml::from_str(&toml).unwrap();
        assert_eq!(parsed, expected);
    }
}

#[test]
fn barebones_package() {
    let toml = "
[package]
name = \"sample-package\"
version = \"0.1.0\"
";

    let expected = RawTomlFields {
        package: Some(Package {
            resolver: None,
            edition: None,
        }),
        workspace: None,
    };

    let parsed: RawTomlFields = toml::from_str(&toml).unwrap();
    assert_eq!(parsed, expected);
}

#[test]
fn barebones_workspace() {
    let toml = "
[workspace]
members = [\"some-package\"]
";

    let expected = RawTomlFields {
        package: None,
        workspace: Some(Workspace {
            package: None,
            resolver: None,
        }),
    };

    let parsed: RawTomlFields = toml::from_str(&toml).unwrap();
    assert_eq!(parsed, expected);
}
