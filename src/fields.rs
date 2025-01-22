use crate::Resolver;

pub(crate) struct TomlFields {
    pub resolver: Option<String>,
    pub edition: Option<String>,
}

impl TomlFields {
    pub fn resolver(&self) -> Resolver {
        if let Some(ver) = &self.resolver {
            match ver.as_str() {
                "1" => Resolver::V1,
                "2" => Resolver::V2,
                "3" => Resolver::V3,
                _ => todo!(),
            }
        } else if let Some(ed) = &self.edition {
            match ed.as_str() {
                "2015" => Resolver::V1,
                "2018" | "2021" => Resolver::V2,
                "2024" => Resolver::V3,
                _ => todo!(),
            }
        } else {
            Resolver::V1
        }
    }
}
